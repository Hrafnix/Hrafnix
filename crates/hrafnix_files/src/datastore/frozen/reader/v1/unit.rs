use crate::common::deserialize::deserialize_object;
use crate::datastore::definition::reader::v1::unit::UnitDefinitionReaderV1;
use hrafnix_datastore::frozen::UnitFrozen;
use serde::Deserialize;

/// Version-1 frozen unit definition and unevaluated selection expression.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UnitReaderV1 {
    /// Unit metadata, default expression, and selection constraints.
    #[serde(deserialize_with = "deserialize_object")]
    definition: UnitDefinitionReaderV1,
    /// Current unevaluated unit selection expression.
    value: String,
}

impl From<UnitReaderV1> for UnitFrozen {
    fn from(unit: UnitReaderV1) -> Self {
        UnitFrozen::new_with_value(unit.definition.into(), unit.value)
    }
}
