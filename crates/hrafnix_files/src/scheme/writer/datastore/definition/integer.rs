use super::common::constraints::ConstraintIntDefinitionWriterV1;
use crate::common::serialize::serialize_shareable_string;
use hrafnix_datastore::definition::{IntegerConstraintEnum, IntegerDefinition};
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 integer expression and bounds.
#[derive(Debug, Serialize)]
pub(crate) struct IntegerDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Default expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    default: ShareableString,
    /// Integer bounds.
    #[serde(with = "ConstraintIntDefinitionWriterV1")]
    constraint: IntegerConstraintEnum,
}

impl From<&IntegerDefinition> for IntegerDefinitionWriterV1 {
    fn from(integer: &IntegerDefinition) -> Self {
        Self {
            description: integer.description(),
            default: integer.default_value(),
            constraint: integer.constraint(),
        }
    }
}
