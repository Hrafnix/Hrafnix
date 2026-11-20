use crate::common::deserialize::deserialize_object;
use crate::datastore::definition::reader::v1::choice::ChoiceDefinitionReaderV1;
use hrafnix_datastore::frozen::ChoiceFrozen;
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

impl From<ChoiceReaderV1> for ChoiceFrozen {
    fn from(choice: ChoiceReaderV1) -> Self {
        ChoiceFrozen::new_with_value(choice.definition.into(), choice.value)
    }
}
