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

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::NumberConstraint;
    use serde_json::json;

    #[test]
    fn writes_expression_and_numeric_bounds() {
        let definition = NumberDefinition::new_with_constraint_and_default(
            "Ratio",
            NumberConstraint::range(-1.5, 2.5, true, false),
            "1 / 2",
        );
        assert_eq!(
            serde_json::to_value(NumberDefinitionWriterV1::from(&definition)).unwrap(),
            json!({
                "description": "Ratio", "default": "1 / 2",
                "constraint": {
                    "type": "range", "min": -1.5, "max": 2.5,
                    "min_inclusive": true, "max_inclusive": false
                }
            })
        );
    }
}
