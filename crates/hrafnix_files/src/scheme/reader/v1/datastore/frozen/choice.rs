use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::choice::ChoiceDefinitionReaderV1;
use hrafnix_datastore::frozen::ChoiceFrozen;
use hrafnix_message::message::Message;
use serde::Deserialize;

/// Version-1 frozen choice definition and unevaluated expression.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ChoiceReaderV1 {
    /// Choice metadata, default expression, and available choices.
    #[serde(deserialize_with = "deserialize_object")]
    definition: ChoiceDefinitionReaderV1,
    /// Current unevaluated choice expression.
    value: String,
}

impl TryFrom<ChoiceReaderV1> for ChoiceFrozen {
    type Error = Message;

    fn try_from(choice: ChoiceReaderV1) -> Result<Self, Self::Error> {
        Ok(ChoiceFrozen::new_with_value(
            choice.definition.try_into()?,
            choice.value,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use serde_json::{Value, json};

    fn document() -> Value {
        json!({
            "definition": {
                "description": "Mode",
                "default": "fast",
                "items": [
                    {"id": "fast", "description": "Fast"},
                    {"id": "slow", "description": "Slow"}
                ]
            },
            "value": "slow"
        })
    }

    #[test]
    fn reads_definition_choices_and_current_value() {
        let reader = serde_json::from_value::<RootObject<ChoiceReaderV1>>(document()).unwrap();
        let choice = ChoiceFrozen::try_from(reader.document).unwrap();

        assert_eq!(choice.value(), "slow");
        assert_eq!(choice.definition().description(), "Mode");
        assert_eq!(choice.definition().default_value(), "fast");
        assert_eq!(choice.definition().ids(), vec!["fast", "slow"]);
        assert_eq!(choice.definition().descriptions(), vec!["Fast", "Slow"]);
    }

    #[test]
    fn rejects_invalid_choice_identifier_during_conversion() {
        let mut value = document();
        value["definition"]["items"][0]["id"] = json!("");
        let reader = serde_json::from_value::<RootObject<ChoiceReaderV1>>(value).unwrap();
        let error = ChoiceFrozen::try_from(reader.document).unwrap_err();

        assert_eq!(
            error.translate_data().message_key().as_str(),
            "datastore_key_empty"
        );
    }
}
