use crate::common::serialize::serialize_shareable_string;
use crate::scheme::writer::datastore::definition::unit::UnitDefinitionWriterV1;
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

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::UnitDefinition;
    use hrafnix_units::UnitFamilyId;
    use serde_json::json;

    #[test]
    fn writes_unit_family_default_and_current_selection() {
        let unit = UnitFrozen::new_with_value(
            UnitDefinition::new_with_default(
                "Distance units",
                UnitFamilyId::Length,
                "length_meter",
            ),
            "length_centimeter",
        );

        assert_eq!(
            serde_json::to_value(UnitWriterV1::from(&unit)).unwrap(),
            json!({
                "definition": {
                    "description": "Distance units", "default": "length_meter",
                    "unit_family": "length"
                },
                "value": "length_centimeter"
            })
        );
    }
}
