use crate::common::serialize::serialize_shareable_string;
use hrafnix_datastore::definition::FileDefinition;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 file expression and dialog options.
#[derive(Debug, Serialize)]
pub(crate) struct FileDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Default expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    default: ShareableString,
    /// Dialog extension filter.
    #[serde(serialize_with = "serialize_shareable_string")]
    extension_filter: ShareableString,
    /// Input/archive flag.
    is_input: bool,
}

impl From<&FileDefinition> for FileDefinitionWriterV1 {
    fn from(file: &FileDefinition) -> Self {
        Self {
            description: file.description(),
            default: file.default_value(),
            extension_filter: file.extension_filter(),
            is_input: file.is_input(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn writes_path_filter_and_archive_option() {
        for is_input in [false, true] {
            let definition =
                FileDefinition::new_with_default("Data", "*.csv", is_input, "samples.csv");
            assert_eq!(
                serde_json::to_value(FileDefinitionWriterV1::from(&definition)).unwrap(),
                json!({
                    "description": "Data", "default": "samples.csv",
                    "extension_filter": "*.csv", "is_input": is_input
                })
            );
        }
    }
}
