use super::common::constraints::ConstraintNumberDefinitionWriterV1;
use super::common::units::UnitIdDefinitionWriterV1;
use crate::common::serialize::serialize_shareable_string;
use hrafnix_datastore::definition::{NumberConstraintEnum, NumberWithUnitsDefinition};
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 numeric expression, bounds, and preferred unit.
#[derive(Debug, Serialize)]
pub(crate) struct NumberWithUnitDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Default expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    default: ShareableString,
    /// Number bounds.
    #[serde(with = "ConstraintNumberDefinitionWriterV1")]
    constraint: NumberConstraintEnum,
    /// Preferred display unit as a frozen version-1 identifier.
    preferred_unit: UnitIdDefinitionWriterV1,
}

impl From<&NumberWithUnitsDefinition> for NumberWithUnitDefinitionWriterV1 {
    fn from(number: &NumberWithUnitsDefinition) -> Self {
        Self {
            description: number.description(),
            default: number.default_value(),
            constraint: number.constraint(),
            preferred_unit: number.preferred_units().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::NumberConstraint;
    use hrafnix_units::UnitId;
    use serde_json::json;

    #[test]
    fn writes_expression_bounds_and_plain_unit_identifier() {
        let definition = NumberWithUnitsDefinition::new_with_constraint_and_default(
            "Distance",
            NumberConstraint::min(0.0, false),
            "12 cm",
            UnitId::Length_Centimeter,
        );
        assert_eq!(
            serde_json::to_value(NumberWithUnitDefinitionWriterV1::from(&definition)).unwrap(),
            json!({
                "description": "Distance", "default": "12 cm",
                "constraint": {"type": "min", "min": 0.0, "inclusive": false},
                "preferred_unit": "length_centimeter"
            })
        );
    }
}
