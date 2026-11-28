use crate::scheme::reader::v1::datastore::definition::common::constraints::ConstraintNumberDefinitionReaderV1;
use crate::scheme::reader::v1::datastore::definition::common::units::UnitIdEnumReaderV1;
use hrafnix_datastore::definition::NumberWithUnitsDefinition;
use serde::Deserialize;

/// Numeric expression, bounds, and preferred unit.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NumberWithUnitDefinitionReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// Number bounds.
    constraint: ConstraintNumberDefinitionReaderV1,
    /// Preferred display unit.
    preferred_unit: UnitIdEnumReaderV1,
}

impl From<NumberWithUnitDefinitionReaderV1> for NumberWithUnitsDefinition {
    fn from(number: NumberWithUnitDefinitionReaderV1) -> Self {
        NumberWithUnitsDefinition::new_with_constraint_and_default(
            number.description,
            number.constraint.into(),
            number.default,
            number.preferred_unit.into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::NumberConstraint;
    use hrafnix_units::UnitId;
    use serde_json::json;

    #[test]
    fn reads_expression_bounds_and_preferred_unit() {
        let reader =
            serde_json::from_value::<RootObject<NumberWithUnitDefinitionReaderV1>>(json!({
                "description": "Distance", "default": "12 cm",
                "constraint": {"type": "min", "min": 0.0, "inclusive": false},
                "preferred_unit": "length_centimeter"
            }))
            .unwrap();
        assert_eq!(
            NumberWithUnitsDefinition::from(reader.document),
            NumberWithUnitsDefinition::new_with_constraint_and_default(
                "Distance",
                NumberConstraint::min(0.0, false),
                "12 cm",
                UnitId::Length_Centimeter
            )
        );
    }

    #[test]
    fn rejects_unknown_unit_identifier() {
        let error = serde_json::from_value::<RootObject<NumberWithUnitDefinitionReaderV1>>(json!({
            "description": "Distance", "default": "12 cm",
            "constraint": {"type": "none"}, "preferred_unit": "unknown"
        }))
        .unwrap_err();
        assert!(error.to_string().contains("unknown variant `unknown`"));
    }
}
