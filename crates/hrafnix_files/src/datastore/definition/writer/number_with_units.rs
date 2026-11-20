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
