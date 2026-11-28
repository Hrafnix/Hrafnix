use crate::scheme::reader::v1::datastore::definition::common::constraints::ConstraintIntDefinitionReaderV1;
use hrafnix_datastore::definition::IntegerDefinition;
use serde::Deserialize;

/// Integer expression and bounds.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IntegerDefinitionReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// Integer bounds.
    constraint: ConstraintIntDefinitionReaderV1,
}

impl From<IntegerDefinitionReaderV1> for IntegerDefinition {
    fn from(integer: IntegerDefinitionReaderV1) -> Self {
        IntegerDefinition::new_with_constraint_and_default(
            integer.description,
            integer.constraint.into(),
            integer.default,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::IntegerConstraint;
    use serde_json::json;

    #[test]
    fn reads_expression_and_bounds() {
        let reader = serde_json::from_value::<RootObject<IntegerDefinitionReaderV1>>(json!({
            "description": "Count", "default": "2 + 3",
            "constraint": {
                "type": "range", "min": -2, "max": 10,
                "min_inclusive": false, "max_inclusive": true
            }
        }))
        .unwrap();
        assert_eq!(
            IntegerDefinition::from(reader.document),
            IntegerDefinition::new_with_constraint_and_default(
                "Count",
                IntegerConstraint::range(-2, 10, false, true),
                "2 + 3"
            )
        );
    }

    #[test]
    fn rejects_fractional_integer_bound() {
        let error = serde_json::from_value::<RootObject<IntegerDefinitionReaderV1>>(json!({
            "description": "Count", "default": "5",
            "constraint": {"type": "min", "min": 0.5, "inclusive": true}
        }))
        .unwrap_err();
        assert!(error.to_string().contains("expected i64"));
    }
}
