use crate::common::serialize::serialize_shareable_string;
use hrafnix_datastore::definition::TabDefinition;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 tab label.
#[derive(Debug, Serialize)]
pub(crate) struct TabWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
}

impl From<&TabDefinition> for TabWriterV1 {
    fn from(tab: &TabDefinition) -> Self {
        Self {
            description: tab.description(),
        }
    }
}
