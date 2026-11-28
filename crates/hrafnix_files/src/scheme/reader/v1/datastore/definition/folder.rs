use hrafnix_datastore::definition::FolderDefinition;
use serde::Deserialize;

/// Version-1 folder expression and archive option.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FolderDefinitionReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// Input/archive flag.
    is_input: bool,
}

impl From<FolderDefinitionReaderV1> for FolderDefinition {
    fn from(folder: FolderDefinitionReaderV1) -> Self {
        FolderDefinition::new_with_default(folder.description, folder.is_input, folder.default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use serde_json::json;

    #[test]
    fn reads_path_and_archive_option() {
        for is_input in [false, true] {
            let reader = serde_json::from_value::<RootObject<FolderDefinitionReaderV1>>(json!({
                "description": "Workspace", "default": "data/input", "is_input": is_input
            }))
            .unwrap();
            assert_eq!(
                FolderDefinition::from(reader.document),
                FolderDefinition::new_with_default("Workspace", is_input, "data/input")
            );
        }
    }

    #[test]
    fn rejects_non_boolean_archive_option() {
        let error = serde_json::from_value::<RootObject<FolderDefinitionReaderV1>>(json!({
            "description": "Workspace", "default": "data/input", "is_input": "true"
        }))
        .unwrap_err();
        assert!(error.to_string().contains("expected a boolean"));
    }
}
