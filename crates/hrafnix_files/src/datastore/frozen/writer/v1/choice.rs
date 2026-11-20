use crate::common::serialize::serialize_shareable_string;
use crate::datastore::definition::writer::choice::ChoiceDefinitionWriterV1;
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
