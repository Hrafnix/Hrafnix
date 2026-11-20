use crate::common::deserialize::deserialize_object;
use crate::datastore::definition::reader::v1::choice::ChoiceDefinitionReaderV1;
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
