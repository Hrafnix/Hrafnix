use crate::common::serialize::serialize_shareable_string;
use crate::scheme::writer::datastore::definition::choice::ChoiceDefinitionWriterV1;
use hrafnix_datastore::frozen::ChoiceFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen choice definition and unevaluated expression.
#[derive(Debug, Serialize)]
pub(crate) struct ChoiceWriterV1 {
    /// Choice metadata, default expression, and available choices.
    definition: ChoiceDefinitionWriterV1,
    /// Current unevaluated choice expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    value: ShareableString,
}

impl From<&ChoiceFrozen> for ChoiceWriterV1 {
    fn from(choice: &ChoiceFrozen) -> Self {
        Self {
            definition: choice.definition().into(),
            value: choice.value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::{ChoiceDefinition, ChoiceItemDefinition};
    use hrafnix_keys::StoreKey;
    use serde_json::json;

    #[test]
    fn writes_ordered_choices_and_current_expression() {
        let choice = ChoiceFrozen::new_with_value(
            ChoiceDefinition::new_with_default(
                "Mode",
                vec![
                    ChoiceItemDefinition::new(StoreKey::new("fast".into()).unwrap(), "Fast"),
                    ChoiceItemDefinition::new(StoreKey::new("slow".into()).unwrap(), "Slow"),
                ],
                "fast",
            ),
            "slow",
        );

        assert_eq!(
            serde_json::to_value(ChoiceWriterV1::from(&choice)).unwrap(),
            json!({
                "definition": {
                    "description": "Mode", "default": "fast",
                    "items": [
                        {"id": "fast", "description": "Fast"},
                        {"id": "slow", "description": "Slow"}
                    ]
                },
                "value": "slow"
            })
        );
    }
}
