use crate::common::serialize::serialize_shareable_string;
use hrafnix_datastore::definition::FolderDefinition;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 folder expression and archive option.
#[derive(Debug, Serialize)]
pub(crate) struct FolderWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Default expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    default: ShareableString,
    /// Input/archive flag.
    is_input: bool,
}

impl From<&FolderDefinition> for FolderWriterV1 {
    fn from(folder: &FolderDefinition) -> Self {
        Self {
            description: folder.description(),
            default: folder.default_value(),
            is_input: folder.is_input(),
        }
    }
}
