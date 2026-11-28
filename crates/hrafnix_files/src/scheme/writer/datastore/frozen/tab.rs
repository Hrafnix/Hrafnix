use crate::scheme::writer::datastore::definition::tab::TabDefinitionWriterV1;
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

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::TabDefinition;
    use serde_json::json;

    #[test]
    fn writes_only_definition_metadata() {
        let tab = TabFrozen::new(TabDefinition::new("Settings"));

        assert_eq!(
            serde_json::to_value(TabWriterV1::from(&tab)).unwrap(),
            json!({"definition": {"description": "Settings"}})
        );
    }
}
