use crate::common::serialize::serialize_shareable_string;
use hrafnix_datastore::definition::FolderDefinition;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 folder expression and archive option.
#[derive(Debug, Serialize)]
pub(crate) struct FolderDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Default expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    default: ShareableString,
    /// Input/archive flag.
    is_input: bool,
}

impl From<&FolderDefinition> for FolderDefinitionWriterV1 {
    fn from(folder: &FolderDefinition) -> Self {
        Self {
            description: folder.description(),
            default: folder.default_value(),
            is_input: folder.is_input(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn writes_path_and_archive_option() {
        for is_input in [false, true] {
            let definition =
                FolderDefinition::new_with_default("Workspace", is_input, "data/input");
            assert_eq!(
                serde_json::to_value(FolderDefinitionWriterV1::from(&definition)).unwrap(),
                json!({
                    "description": "Workspace", "default": "data/input", "is_input": is_input
                })
            );
        }
    }
}
