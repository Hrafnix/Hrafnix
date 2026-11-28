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

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::IntegerConstraint;
    use serde_json::json;

    #[test]
    fn writes_expression_and_integer_bounds() {
        let definition = IntegerDefinition::new_with_constraint_and_default(
            "Count",
            IntegerConstraint::range(-2, 10, false, true),
            "2 + 3",
        );
        assert_eq!(
            serde_json::to_value(IntegerDefinitionWriterV1::from(&definition)).unwrap(),
            json!({
                "description": "Count", "default": "2 + 3",
                "constraint": {
                    "type": "range", "min": -2, "max": 10,
                    "min_inclusive": false, "max_inclusive": true
                }
            })
        );
    }
}
