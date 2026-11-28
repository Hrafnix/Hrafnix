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
