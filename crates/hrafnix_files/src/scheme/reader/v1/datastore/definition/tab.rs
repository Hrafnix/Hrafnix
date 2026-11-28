use hrafnix_datastore::definition::TabDefinition;
use serde::Deserialize;

/// Version-1 tab label.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TabDefinitionReaderV1 {
    /// Item description.
    description: String,
}

impl From<TabDefinitionReaderV1> for TabDefinition {
    fn from(tab: TabDefinitionReaderV1) -> Self {
        TabDefinition::new(tab.description)
    }
}
