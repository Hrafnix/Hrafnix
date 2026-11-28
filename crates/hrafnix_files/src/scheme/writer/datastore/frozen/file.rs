use crate::common::serialize::serialize_shareable_string;
use crate::scheme::writer::datastore::definition::file::FileDefinitionWriterV1;
use hrafnix_datastore::frozen::FileFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen file definition and unevaluated path expression.
#[derive(Debug, Serialize)]
pub(crate) struct FileWriterV1 {
    /// File metadata, default expression, and dialog options.
    definition: FileDefinitionWriterV1,
    /// Current unevaluated file path expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    value: ShareableString,
}

impl From<&FileFrozen> for FileWriterV1 {
    fn from(file: &FileFrozen) -> Self {
        Self {
            definition: file.definition().into(),
            value: file.value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::FileDefinition;
    use serde_json::json;

    #[test]
    fn writes_path_expression_and_dialog_options() {
        let file = FileFrozen::new_with_value(
            FileDefinition::new_with_default("Input file", "*.csv", true, "default.csv"),
            "p_filename",
        );

        assert_eq!(
            serde_json::to_value(FileWriterV1::from(&file)).unwrap(),
            json!({
                "definition": {
                    "description": "Input file", "default": "default.csv",
                    "extension_filter": "*.csv", "is_input": true
                },
                "value": "p_filename"
            })
        );
    }
}
