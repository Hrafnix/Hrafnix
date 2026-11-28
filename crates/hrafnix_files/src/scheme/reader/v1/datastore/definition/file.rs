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
