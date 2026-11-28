use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::tab::TabDefinitionReaderV1;
use hrafnix_datastore::frozen::TabFrozen;
use serde::Deserialize;

/// Version-1 frozen tab, which has metadata but no value.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TabReaderV1 {
    /// Tab label.
    #[serde(deserialize_with = "deserialize_object")]
    definition: TabDefinitionReaderV1,
}

impl From<TabReaderV1> for TabFrozen {
    fn from(tab: TabReaderV1) -> Self {
        TabFrozen::new(tab.definition.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::TabDefinition;
    use serde_json::json;

    #[test]
    fn reads_metadata_without_value() {
        let reader = serde_json::from_value::<RootObject<TabReaderV1>>(json!({
            "definition": {"description": "Settings"}
        }))
        .unwrap();
        let tab = TabFrozen::from(reader.document);

        assert_eq!(tab.definition(), &TabDefinition::new("Settings"));
    }

    #[test]
    fn rejects_value_on_structural_item() {
        assert!(
            serde_json::from_value::<RootObject<TabReaderV1>>(json!({
                "definition": {"description": "Settings"},
                "value": ""
            }))
            .is_err()
        );
    }
}
