use crate::common::serialize::serialize_shareable_string;
use crate::datastore::definition::writer::number::NumberDefinitionWriterV1;
use hrafnix_datastore::frozen::NumberFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen number definition and unevaluated expression.
#[derive(Debug, Serialize)]
pub(crate) struct NumberWriterV1 {
    /// Numeric metadata, default expression, and bounds.
    definition: NumberDefinitionWriterV1,
    /// Current unevaluated numeric expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    value: ShareableString,
}

impl From<&NumberFrozen> for NumberWriterV1 {
    fn from(number: &NumberFrozen) -> Self {
        Self {
            definition: number.definition().into(),
            value: number.value(),
        }
    }
}
