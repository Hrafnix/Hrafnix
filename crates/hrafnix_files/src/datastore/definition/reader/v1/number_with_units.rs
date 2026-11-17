use crate::datastore::definition::reader::v1::common::constraints::ConstraintNumberReaderV1;
use crate::datastore::definition::reader::v1::common::units::UnitIdEnumReaderV1;
use hrafnix_datastore::definition::NumberWithUnitsDefinition;
use serde::Deserialize;

/// Numeric expression, bounds, and preferred unit.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NumberWithUnitReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// Number bounds.
    constraint: ConstraintNumberReaderV1,
    /// Preferred display unit.
    preferred_unit: UnitIdEnumReaderV1,
}

impl From<NumberWithUnitReaderV1> for NumberWithUnitsDefinition {
    fn from(number: NumberWithUnitReaderV1) -> Self {
        NumberWithUnitsDefinition::new_with_constraint_and_default(
            number.description,
            number.constraint.into(),
            number.default,
            number.preferred_unit.into(),
        )
    }
}
