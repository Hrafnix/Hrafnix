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
