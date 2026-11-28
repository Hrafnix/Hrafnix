use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::integer::IntegerDefinitionReaderV1;
use hrafnix_datastore::frozen::IntegerFrozen;
use serde::Deserialize;

/// Version-1 frozen integer definition and unevaluated expression.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IntegerReaderV1 {
    /// Integer metadata, default expression, and bounds.
    #[serde(deserialize_with = "deserialize_object")]
    definition: IntegerDefinitionReaderV1,
    /// Current unevaluated integer expression.
    value: String,
}

impl From<IntegerReaderV1> for IntegerFrozen {
    fn from(integer: IntegerReaderV1) -> Self {
        IntegerFrozen::new_with_value(integer.definition.into(), integer.value)
    }
}
