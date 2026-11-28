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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use serde_json::json;

    #[test]
    fn reads_tab_label() {
        let reader = serde_json::from_value::<RootObject<TabDefinitionReaderV1>>(json!({
            "description": "Advanced settings"
        }))
        .unwrap();
        assert_eq!(
            TabDefinition::from(reader.document),
            TabDefinition::new("Advanced settings")
        );
    }

    #[test]
    fn rejects_value_fields() {
        let error = serde_json::from_value::<RootObject<TabDefinitionReaderV1>>(json!({
            "description": "Advanced settings", "default": "unused"
        }))
        .unwrap_err();
        assert!(error.to_string().contains("unknown field `default`"));
    }
}
