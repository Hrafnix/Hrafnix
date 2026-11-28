use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::folder::FolderDefinitionReaderV1;
use hrafnix_datastore::frozen::FolderFrozen;
use serde::Deserialize;

/// Version-1 frozen folder definition and unevaluated path expression.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FolderReaderV1 {
    /// Folder metadata, default expression, and archive option.
    #[serde(deserialize_with = "deserialize_object")]
    definition: FolderDefinitionReaderV1,
    /// Current unevaluated folder path expression.
    value: String,
}

impl From<FolderReaderV1> for FolderFrozen {
    fn from(folder: FolderReaderV1) -> Self {
        FolderFrozen::new_with_value(folder.definition.into(), folder.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::FolderDefinition;
    use serde_json::{Value, json};

    fn document() -> Value {
        json!({
            "definition": {
                "description": "Output folder",
                "default": "results",
                "is_input": false
            },
            "value": "p_folder"
        })
    }

    #[test]
    fn reads_definition_and_path_expression() {
        let reader = serde_json::from_value::<RootObject<FolderReaderV1>>(document()).unwrap();
        let folder = FolderFrozen::from(reader.document);

        assert_eq!(folder.value(), "p_folder");
        assert_eq!(
            folder.definition(),
            &FolderDefinition::new_with_default("Output folder", false, "results")
        );
    }

    #[test]
    fn rejects_unknown_fields() {
        let mut value = document();
        value["extension_filter"] = json!("*");
        assert!(serde_json::from_value::<RootObject<FolderReaderV1>>(value).is_err());
    }
}
