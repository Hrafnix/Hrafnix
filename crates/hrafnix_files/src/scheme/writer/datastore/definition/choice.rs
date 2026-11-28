use crate::common::serialize::{serialize_shareable_string, serialize_store_key};
use hrafnix_datastore::definition::{ChoiceDefinition, ChoiceItemDefinition};
use hrafnix_keys::StoreKey;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 choice identifier and label.
#[derive(Debug, Serialize)]
pub(crate) struct ChoiceItemDefinitionWriterV1 {
    /// Plain-string choice identifier.
    #[serde(serialize_with = "serialize_store_key")]
    id: StoreKey,
    /// Choice label.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
}

impl From<&ChoiceItemDefinition> for ChoiceItemDefinitionWriterV1 {
    fn from(item: &ChoiceItemDefinition) -> Self {
        Self {
            id: item.id(),
            description: item.description(),
        }
    }
}

/// Version-1 choice expression and ordered choices.
#[derive(Debug, Serialize)]
pub(crate) struct ChoiceDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Default expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    default: ShareableString,
    /// Choice items.
    items: Vec<ChoiceItemDefinitionWriterV1>,
}

impl From<&ChoiceDefinition> for ChoiceDefinitionWriterV1 {
    fn from(choice: &ChoiceDefinition) -> Self {
        Self {
            description: choice.description(),
            default: choice.default_value(),
            items: choice.choices().iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn writes_choice_items_in_definition_order() {
        let first = ChoiceItemDefinition::new(StoreKey::new("slow".into()).unwrap(), "Slow");
        assert_eq!(
            serde_json::to_value(ChoiceItemDefinitionWriterV1::from(&first)).unwrap(),
            json!({"id": "slow", "description": "Slow"})
        );
        let definition = ChoiceDefinition::new_with_default(
            "Mode",
            vec![
                first,
                ChoiceItemDefinition::new(StoreKey::new("fast".into()).unwrap(), "Fast"),
            ],
            "fast",
        );
        assert_eq!(
            serde_json::to_value(ChoiceDefinitionWriterV1::from(&definition)).unwrap(),
            json!({
                "description": "Mode", "default": "fast",
                "items": [
                    {"id": "slow", "description": "Slow"},
                    {"id": "fast", "description": "Fast"}
                ]
            })
        );
    }
}
