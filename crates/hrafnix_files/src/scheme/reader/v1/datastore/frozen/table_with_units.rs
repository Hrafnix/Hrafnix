use crate::common::deserialize::deserialize_object;
use crate::common::validate::validate_unit_count;
use crate::scheme::reader::v1::datastore::definition::table_with_units::TableWithUnitsDefinitionReaderV1;
use hrafnix_datastore::frozen::TableWithUnitsFrozen;
use hrafnix_message::message::Message;
use hrafnix_shareable_string::ShareableString;
use serde::Deserialize;

/// Version-1 frozen table definition, cell expressions, and column units.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TableWithUnitsReaderV1 {
    /// Table metadata, ordered unit-aware columns, and default rows.
    #[serde(deserialize_with = "deserialize_object")]
    definition: TableWithUnitsDefinitionReaderV1,
    /// Unevaluated table reference, or an empty string for local rows.
    parameter: String,
    /// Selected unit strings in definition column order.
    units: Vec<String>,
    /// Row-major cell expressions in definition column order.
    rows: Vec<Vec<String>>,
}

impl TryFrom<TableWithUnitsReaderV1> for TableWithUnitsFrozen {
    type Error = Message;

    fn try_from(table: TableWithUnitsReaderV1) -> Result<Self, Self::Error> {
        validate_unit_count(table.units.len(), table.definition.column_count())?;
        let units = table.units.into_iter().map(ShareableString::from).collect();
        let rows = table
            .rows
            .into_iter()
            .map(|row| row.into_iter().map(ShareableString::from).collect())
            .collect();
        Ok(TableWithUnitsFrozen::new_from_rows_with_parameter(
            table.definition.try_into()?,
            rows,
            units,
            table.parameter,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::{
        NumberConstraint, NumberWithUnitsDefinition, TableColumnRequirement,
        TableWithUnitsDefinition,
    };
    use hrafnix_keys::StoreKey;
    use hrafnix_units::UnitId;
    use serde_json::{Value, json};

    fn document() -> Value {
        json!({
            "definition": {
                "description": "Measurements",
                "columns": [
                    {
                        "key": "z", "description": "Distance", "default": "1 m",
                        "constraint": {"type": "min", "min": 0.0, "inclusive": true},
                        "requirement": "increasing", "preferred_unit": "length_meter"
                    },
                    {
                        "key": "a", "description": "Ratio", "default": "1",
                        "constraint": {"type": "none"}, "requirement": "none",
                        "preferred_unit": "none"
                    }
                ],
                "default": [["1 m", "1"]]
            },
            "parameter": "p_source",
            "units": ["length_centimeter", "none"],
            "rows": [["2 * p_length", "3"], ["4 m", "5"]]
        })
    }

    #[test]
    fn reads_definition_reference_rows_and_one_unit_per_column() {
        let reader =
            serde_json::from_value::<RootObject<TableWithUnitsReaderV1>>(document()).unwrap();
        let table = TableWithUnitsFrozen::try_from(reader.document).unwrap();
        let expected_definition =
            TableWithUnitsDefinition::new_with_column_requirements_and_default(
                "Measurements",
                vec![
                    (
                        StoreKey::new("z".into()).unwrap(),
                        NumberWithUnitsDefinition::new_with_constraint_and_default(
                            "Distance",
                            NumberConstraint::min(0.0, true),
                            "1 m",
                            UnitId::Length_Meter,
                        ),
                        TableColumnRequirement::Increasing,
                    ),
                    (
                        StoreKey::new("a".into()).unwrap(),
                        NumberWithUnitsDefinition::new_with_default("Ratio", "1", UnitId::None),
                        TableColumnRequirement::None,
                    ),
                ],
                vec![vec!["1 m", "1"]],
            );

        assert_eq!(table.definition(), &expected_definition);
        assert_eq!(table.parameter(), "p_source");
        assert_eq!(
            table.rows(),
            &[
                vec![ShareableString::from("2 * p_length"), "3".into()],
                vec!["4 m".into(), "5".into()]
            ]
        );
        assert_eq!(
            table.units(),
            &[ShareableString::from("length_centimeter"), "none".into()]
        );
        assert_eq!(table.unit_by_name("z").unwrap(), "length_centimeter");
    }

    #[test]
    fn rejects_unit_count_mismatch_during_conversion() {
        let mut value = document();
        value["units"] = json!(["length_centimeter"]);
        let reader = serde_json::from_value::<RootObject<TableWithUnitsReaderV1>>(value).unwrap();
        let error = TableWithUnitsFrozen::try_from(reader.document).unwrap_err();
        let data = error.translate_data();

        assert_eq!(
            data.message_key().as_str(),
            "files_table_unit_count_mismatch"
        );
        assert_eq!(data.message_params()["units"], "1");
        assert_eq!(data.message_params()["columns"], "2");
    }

    #[test]
    fn requires_explicit_parameter_even_for_local_rows() {
        let mut value = document();
        value.as_object_mut().unwrap().remove("parameter");
        assert!(serde_json::from_value::<RootObject<TableWithUnitsReaderV1>>(value).is_err());
    }
}
