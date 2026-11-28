use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::boolean::BooleanDefinitionReaderV1;
use hrafnix_datastore::frozen::BooleanFrozen;
use serde::Deserialize;

/// Version-1 frozen boolean definition and unevaluated expression.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BooleanReaderV1 {
    /// Boolean metadata, default expression, and state labels.
    #[serde(deserialize_with = "deserialize_object")]
    definition: BooleanDefinitionReaderV1,
    /// Current unevaluated boolean expression.
    value: String,
}

impl From<BooleanReaderV1> for BooleanFrozen {
    fn from(boolean: BooleanReaderV1) -> Self {
        BooleanFrozen::new_with_value(boolean.definition.into(), boolean.value)
    }
}
