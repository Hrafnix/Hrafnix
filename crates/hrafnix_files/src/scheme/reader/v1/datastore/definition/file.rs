use hrafnix_datastore::definition::FileDefinition;
use serde::Deserialize;

/// File expression and dialog options.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FileDefinitionReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// Dialog extension filter.
    extension_filter: String,
    /// Input/archive flag.
    is_input: bool,
}

impl From<FileDefinitionReaderV1> for FileDefinition {
    fn from(file: FileDefinitionReaderV1) -> Self {
        FileDefinition::new_with_default(
            file.description,
            file.extension_filter,
            file.is_input,
            file.default,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use serde_json::json;

    #[test]
    fn reads_path_and_dialog_options() {
        for is_input in [false, true] {
            let reader = serde_json::from_value::<RootObject<FileDefinitionReaderV1>>(json!({
                "description": "Data", "default": "samples.csv",
                "extension_filter": "*.csv", "is_input": is_input
            }))
            .unwrap();
            assert_eq!(
                FileDefinition::from(reader.document),
                FileDefinition::new_with_default("Data", "*.csv", is_input, "samples.csv")
            );
        }
    }

    #[test]
    fn requires_extension_filter() {
        let error = serde_json::from_value::<RootObject<FileDefinitionReaderV1>>(json!({
            "description": "Data", "default": "samples.csv", "is_input": true
        }))
        .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("missing field `extension_filter`")
        );
    }
}
