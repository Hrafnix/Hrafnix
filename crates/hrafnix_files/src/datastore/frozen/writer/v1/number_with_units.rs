use crate::common::serialize::serialize_shareable_string;
use crate::datastore::definition::writer::number_with_units::NumberWithUnitDefinitionWriterV1;
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
