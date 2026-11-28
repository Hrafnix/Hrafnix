use crate::scheme::reader::v1::datastore::definition::common::constraints::ConstraintNumberDefinitionReaderV1;
use hrafnix_datastore::definition::NumberDefinition;
use serde::Deserialize;

/// Numeric expression and bounds.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NumberDefinitionReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// Number bounds.
    constraint: ConstraintNumberDefinitionReaderV1,
}

impl From<NumberDefinitionReaderV1> for NumberDefinition {
    fn from(number: NumberDefinitionReaderV1) -> Self {
        NumberDefinition::new_with_constraint_and_default(
            number.description,
            number.constraint.into(),
            number.default,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::NumberConstraint;
    use serde_json::json;

    #[test]
    fn reads_expression_and_bounds() {
        let reader = serde_json::from_value::<RootObject<NumberDefinitionReaderV1>>(json!({
            "description": "Ratio", "default": "1 / 2",
            "constraint": {
                "type": "range", "min": -1.5, "max": 2.5,
                "min_inclusive": true, "max_inclusive": false
            }
        }))
        .unwrap();
        assert_eq!(
            NumberDefinition::from(reader.document),
            NumberDefinition::new_with_constraint_and_default(
                "Ratio",
                NumberConstraint::range(-1.5, 2.5, true, false),
                "1 / 2"
            )
        );
    }

    #[test]
    fn rejects_unknown_fields() {
        let error = serde_json::from_value::<RootObject<NumberDefinitionReaderV1>>(json!({
            "description": "Ratio", "default": "0.5",
            "constraint": {"type": "none"}, "unexpected": true
        }))
        .unwrap_err();
        assert!(error.to_string().contains("unknown field `unexpected`"));
    }
}
