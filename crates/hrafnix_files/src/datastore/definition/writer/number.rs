use super::common::constraints::ConstraintNumberDefinitionWriterV1;
use crate::common::serialize::serialize_shareable_string;
use hrafnix_datastore::definition::{NumberConstraintEnum, NumberDefinition};
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 numeric expression and bounds.
#[derive(Debug, Serialize)]
pub(crate) struct NumberDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Default expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    default: ShareableString,
    /// Number bounds.
    #[serde(with = "ConstraintNumberDefinitionWriterV1")]
    constraint: NumberConstraintEnum,
}

impl From<&NumberDefinition> for NumberDefinitionWriterV1 {
    fn from(number: &NumberDefinition) -> Self {
        Self {
            description: number.description(),
            default: number.default_value(),
            constraint: number.constraint(),
        }
    }
}
