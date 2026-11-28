use crate::common::serialize::serialize_shareable_string;
use crate::scheme::writer::datastore::definition::number_with_units::NumberWithUnitDefinitionWriterV1;
use hrafnix_datastore::frozen::NumberWithUnitsFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen number definition, expression, and selected units.
#[derive(Debug, Serialize)]
pub(crate) struct NumberWithUnitsWriterV1 {
    /// Numeric metadata, default expression, constraints, and preferred unit.
    definition: NumberWithUnitDefinitionWriterV1,
    /// Current unevaluated numeric expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    value: ShareableString,
    /// Selected unit string.
    #[serde(serialize_with = "serialize_shareable_string")]
    units: ShareableString,
}

impl From<&NumberWithUnitsFrozen> for NumberWithUnitsWriterV1 {
    fn from(number: &NumberWithUnitsFrozen) -> Self {
        Self {
            definition: number.definition().into(),
            value: number.value(),
            units: number.units(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::{NumberConstraint, NumberWithUnitsDefinition};
    use hrafnix_units::UnitId;
    use serde_json::json;

    #[test]
    fn writes_expression_and_selected_units_separately_from_preferred_units() {
        let number = NumberWithUnitsFrozen::new_with_value(
            NumberWithUnitsDefinition::new_with_constraint_and_default(
                "Distance",
                NumberConstraint::min(0.0, true),
                "1 m",
                UnitId::Length_Meter,
            ),
            "2 * p_length",
            "length_centimeter",
        );

        assert_eq!(
            serde_json::to_value(NumberWithUnitsWriterV1::from(&number)).unwrap(),
            json!({
                "definition": {
                    "description": "Distance", "default": "1 m",
                    "constraint": {"type": "min", "min": 0.0, "inclusive": true},
                    "preferred_unit": "length_meter"
                },
                "value": "2 * p_length",
                "units": "length_centimeter"
            })
        );
    }
}
