use super::common::constraints::ConstraintNumberWriterV1;
use super::common::units::UnitIdWriterV1;
use crate::common::serialize::serialize_shareable_string;
use hrafnix_datastore::definition::{NumberConstraintEnum, NumberWithUnitsDefinition};
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 numeric expression, bounds, and preferred unit.
#[derive(Debug, Serialize)]
pub(crate) struct NumberWithUnitWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Default expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    default: ShareableString,
    /// Number bounds.
    #[serde(with = "ConstraintNumberWriterV1")]
    constraint: NumberConstraintEnum,
    /// Preferred display unit as a frozen version-1 identifier.
    preferred_unit: UnitIdWriterV1,
}

impl From<&NumberWithUnitsDefinition> for NumberWithUnitWriterV1 {
    fn from(number: &NumberWithUnitsDefinition) -> Self {
        Self {
            description: number.description(),
            default: number.default_value(),
            constraint: number.constraint(),
            preferred_unit: number.preferred_units().into(),
        }
    }
}
