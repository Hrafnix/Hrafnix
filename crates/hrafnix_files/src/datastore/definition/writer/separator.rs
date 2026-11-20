use crate::common::serialize::serialize_shareable_string;
use hrafnix_datastore::definition::SeparatorDefinition;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 separator label.
#[derive(Debug, Serialize)]
pub(crate) struct SeparatorDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
}

impl From<&SeparatorDefinition> for SeparatorDefinitionWriterV1 {
    fn from(separator: &SeparatorDefinition) -> Self {
        Self {
            description: separator.description(),
        }
    }
}
