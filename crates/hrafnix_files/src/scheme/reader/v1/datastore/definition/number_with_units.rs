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
