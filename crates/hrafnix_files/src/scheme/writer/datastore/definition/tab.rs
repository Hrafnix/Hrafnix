use crate::common::serialize::serialize_shareable_string;
use hrafnix_datastore::definition::TabDefinition;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 tab label.
#[derive(Debug, Serialize)]
pub(crate) struct TabDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
}

impl From<&TabDefinition> for TabDefinitionWriterV1 {
    fn from(tab: &TabDefinition) -> Self {
        Self {
            description: tab.description(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn writes_only_the_tab_label() {
        let definition = TabDefinition::new("Advanced settings");
        assert_eq!(
            serde_json::to_value(TabDefinitionWriterV1::from(&definition)).unwrap(),
            json!({"description": "Advanced settings"})
        );
    }
}
