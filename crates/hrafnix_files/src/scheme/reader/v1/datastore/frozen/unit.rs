use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::unit::UnitDefinitionReaderV1;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::UnitDefinition;
    use hrafnix_units::UnitFamilyId;
    use serde_json::{Value, json};

    fn document() -> Value {
        json!({
            "definition": {
                "description": "Distance units", "default": "length_meter", "unit_family": "length"
            },
            "value": "length_centimeter"
        })
    }

    #[test]
    fn reads_definition_family_and_current_selection() {
        let reader = serde_json::from_value::<RootObject<UnitReaderV1>>(document()).unwrap();
        let unit = UnitFrozen::from(reader.document);

        assert_eq!(unit.value(), "length_centimeter");
        assert_eq!(
            unit.definition(),
            &UnitDefinition::new_with_default(
                "Distance units",
                UnitFamilyId::Length,
                "length_meter"
            )
        );
    }

    #[test]
    fn rejects_unknown_unit_family() {
        let mut value = document();
        value["definition"]["unit_family"] = json!("unknown");
        assert!(serde_json::from_value::<RootObject<UnitReaderV1>>(value).is_err());
    }
}
