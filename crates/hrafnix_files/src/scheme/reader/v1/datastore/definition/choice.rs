use crate::common::deserialize::deserialize_objects;
use hrafnix_datastore::definition::{ChoiceDefinition, ChoiceItemDefinition};
use hrafnix_keys::StoreKey;
use hrafnix_message::message::Message;
use serde::Deserialize;

/// Version-1 choice item.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ChoiceItemDefinitionReaderV1 {
    /// Plain-string choice identifier, validated during conversion.
    id: String,
    /// Choice label.
    description: String,
}

/// Version-1 choice definition.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ChoiceDefinitionReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// Ordered choices; the datastore definition normalizes duplicate IDs.
    #[serde(deserialize_with = "deserialize_objects")]
    items: Vec<ChoiceItemDefinitionReaderV1>,
}

impl TryFrom<ChoiceDefinitionReaderV1> for ChoiceDefinition {
    type Error = Message;

    fn try_from(choice: ChoiceDefinitionReaderV1) -> Result<Self, Self::Error> {
        let items = choice
            .items
            .into_iter()
            .map(|item| {
                Ok(ChoiceItemDefinition::new(
                    StoreKey::new(item.id.into())?,
                    item.description,
                ))
            })
            .collect::<Result<_, Message>>()?;
        Ok(ChoiceDefinition::new_with_default(
            choice.description,
            items,
            choice.default,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use serde_json::json;

    #[test]
    fn normalizes_duplicate_choice_ids_through_the_definition() {
        let value = json!({
            "description": "Mode",
            "default": "fast",
            "items": [
                {"id": "fast", "description": "Fast"},
                {"id": "slow", "description": "Slow"},
                {"id": "fast", "description": "Also fast"}
            ]
        });
        let reader = serde_json::from_value::<RootObject<ChoiceDefinitionReaderV1>>(value).unwrap();
        let choice = ChoiceDefinition::try_from(reader.document).unwrap();
        assert_eq!(choice.description(), "Mode");
        assert_eq!(choice.ids(), vec!["slow", "fast"]);
        assert_eq!(choice.descriptions(), vec!["Slow", "Also fast"]);
        assert_eq!(choice.default_value(), "fast");
    }

    #[test]
    fn rejects_invalid_choice_identifier_during_conversion() {
        let reader = serde_json::from_value::<RootObject<ChoiceDefinitionReaderV1>>(json!({
            "description": "Mode", "default": "fast",
            "items": [{"id": "invalid key", "description": "Invalid"}]
        }))
        .unwrap();
        let error = ChoiceDefinition::try_from(reader.document).unwrap_err();
        let expected = StoreKey::new("invalid key".into()).unwrap_err();
        assert_eq!(error.translate_data(), expected.translate_data());
    }
}
