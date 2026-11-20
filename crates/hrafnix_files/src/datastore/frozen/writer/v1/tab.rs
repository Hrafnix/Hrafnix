use crate::datastore::definition::writer::tab::TabDefinitionWriterV1;
use hrafnix_datastore::frozen::TabFrozen;
use serde::Serialize;

/// Version-1 frozen tab, which has metadata but no value.
#[derive(Debug, Serialize)]
pub(crate) struct TabWriterV1 {
    /// Tab label.
    definition: TabDefinitionWriterV1,
}

impl From<&TabFrozen> for TabWriterV1 {
    fn from(tab: &TabFrozen) -> Self {
        Self {
            definition: tab.definition().into(),
        }
    }
}
