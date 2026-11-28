use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::file::FileDefinitionReaderV1;
use hrafnix_datastore::frozen::FileFrozen;
use serde::Deserialize;

/// Version-1 frozen file definition and unevaluated path expression.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FileReaderV1 {
    /// File metadata, default expression, and dialog options.
    #[serde(deserialize_with = "deserialize_object")]
    definition: FileDefinitionReaderV1,
    /// Current unevaluated file path expression.
    value: String,
}

impl From<FileReaderV1> for FileFrozen {
    fn from(file: FileReaderV1) -> Self {
        FileFrozen::new_with_value(file.definition.into(), file.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::FileDefinition;
    use serde_json::{Value, json};

    fn document() -> Value {
        json!({
            "definition": {
                "description": "Input file",
                "default": "default.csv",
                "extension_filter": "*.csv",
                "is_input": true
            },
            "value": "p_filename"
        })
    }

    #[test]
    fn reads_definition_and_path_expression() {
        let reader = serde_json::from_value::<RootObject<FileReaderV1>>(document()).unwrap();
        let file = FileFrozen::from(reader.document);

        assert_eq!(file.value(), "p_filename");
        assert_eq!(
            file.definition(),
            &FileDefinition::new_with_default("Input file", "*.csv", true, "default.csv")
        );
    }

    #[test]
    fn rejects_non_boolean_input_option() {
        let mut value = document();
        value["definition"]["is_input"] = json!("true");
        assert!(serde_json::from_value::<RootObject<FileReaderV1>>(value).is_err());
    }
}
