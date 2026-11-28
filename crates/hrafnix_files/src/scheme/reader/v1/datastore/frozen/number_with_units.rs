use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::number_with_units::NumberWithUnitDefinitionReaderV1;
use hrafnix_datastore::frozen::NumberWithUnitsFrozen;
use serde::Deserialize;

/// Version-1 frozen number definition, expression, and selected units.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NumberWithUnitsReaderV1 {
    /// Numeric metadata, default expression, constraints, and preferred unit.
    #[serde(deserialize_with = "deserialize_object")]
    definition: NumberWithUnitDefinitionReaderV1,
    /// Current unevaluated numeric expression.
    value: String,
    /// Selected unit string.
    units: String,
}

impl From<NumberWithUnitsReaderV1> for NumberWithUnitsFrozen {
    fn from(number: NumberWithUnitsReaderV1) -> Self {
        NumberWithUnitsFrozen::new_with_value(number.definition.into(), number.value, number.units)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::{NumberConstraint, NumberWithUnitsDefinition};
    use hrafnix_units::UnitId;
    use serde_json::{Value, json};

    fn document() -> Value {
        json!({
            "definition": {
                "description": "Distance",
                "default": "1 m",
                "constraint": {"type": "min", "min": 0.0, "inclusive": true},
                "preferred_unit": "length_meter"
            },
            "value": "2 * p_length",
            "units": "length_centimeter"
        })
    }

    #[test]
    fn reads_expression_and_selected_units_separately_from_default_units() {
        let reader =
            serde_json::from_value::<RootObject<NumberWithUnitsReaderV1>>(document()).unwrap();
        let number = NumberWithUnitsFrozen::from(reader.document);

        assert_eq!(number.value(), "2 * p_length");
        assert_eq!(number.units(), "length_centimeter");
        assert_eq!(
            number.definition(),
            &NumberWithUnitsDefinition::new_with_constraint_and_default(
                "Distance",
                NumberConstraint::min(0.0, true),
                "1 m",
                UnitId::Length_Meter
            )
        );
    }

    #[test]
    fn rejects_array_instead_of_selected_unit_string() {
        let mut value = document();
        value["units"] = json!(["length_centimeter"]);
        assert!(serde_json::from_value::<RootObject<NumberWithUnitsReaderV1>>(value).is_err());
    }
}
