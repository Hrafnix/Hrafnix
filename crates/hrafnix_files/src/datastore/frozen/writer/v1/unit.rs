use crate::common::serialize::serialize_shareable_string;
use crate::datastore::definition::writer::unit::UnitDefinitionWriterV1;
use hrafnix_datastore::frozen::UnitFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen unit definition and unevaluated selection expression.
#[derive(Debug, Serialize)]
pub(crate) struct UnitWriterV1 {
    /// Unit metadata, default expression, and selection constraints.
    definition: UnitDefinitionWriterV1,
    /// Current unevaluated unit selection expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    value: ShareableString,
}

impl From<&UnitFrozen> for UnitWriterV1 {
    fn from(unit: &UnitFrozen) -> Self {
        Self {
            definition: unit.definition().into(),
            value: unit.value(),
        }
    }
}
