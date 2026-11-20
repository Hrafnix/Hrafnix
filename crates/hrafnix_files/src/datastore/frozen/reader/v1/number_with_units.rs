use crate::common::deserialize::deserialize_object;
use crate::datastore::definition::reader::v1::number_with_units::NumberWithUnitDefinitionReaderV1;
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
