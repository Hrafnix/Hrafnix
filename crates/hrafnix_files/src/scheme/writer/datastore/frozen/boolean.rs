use crate::common::serialize::serialize_shareable_string;
use crate::scheme::writer::datastore::definition::boolean::BooleanDefinitionWriterV1;
use hrafnix_datastore::frozen::BooleanFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen boolean definition and unevaluated expression.
#[derive(Debug, Serialize)]
pub(crate) struct BooleanWriterV1 {
    /// Boolean metadata, default expression, and state labels.
    definition: BooleanDefinitionWriterV1,
    /// Current unevaluated boolean expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    value: ShareableString,
}

impl From<&BooleanFrozen> for BooleanWriterV1 {
    fn from(boolean: &BooleanFrozen) -> Self {
        Self {
            definition: boolean.definition().into(),
            value: boolean.value(),
        }
    }
}
