use crate::common::serialize::serialize_shareable_string;
use crate::datastore::definition::writer::integer::IntegerDefinitionWriterV1;
use hrafnix_datastore::frozen::IntegerFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen integer definition and unevaluated expression.
#[derive(Debug, Serialize)]
pub(crate) struct IntegerWriterV1 {
    /// Integer metadata, default expression, and bounds.
    definition: IntegerDefinitionWriterV1,
    /// Current unevaluated integer expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    value: ShareableString,
}

impl From<&IntegerFrozen> for IntegerWriterV1 {
    fn from(integer: &IntegerFrozen) -> Self {
        Self {
            definition: integer.definition().into(),
            value: integer.value(),
        }
    }
}
