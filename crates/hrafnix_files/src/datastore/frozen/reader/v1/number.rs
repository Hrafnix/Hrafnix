use crate::common::deserialize::deserialize_object;
use crate::datastore::definition::reader::v1::number::NumberDefinitionReaderV1;
use hrafnix_datastore::frozen::NumberFrozen;
use serde::Deserialize;

/// Version-1 frozen number definition and unevaluated expression.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NumberReaderV1 {
    /// Numeric metadata, default expression, and bounds.
    #[serde(deserialize_with = "deserialize_object")]
    definition: NumberDefinitionReaderV1,
    /// Current unevaluated numeric expression.
    value: String,
}

impl From<NumberReaderV1> for NumberFrozen {
    fn from(number: NumberReaderV1) -> Self {
        NumberFrozen::new_with_value(number.definition.into(), number.value)
    }
}
