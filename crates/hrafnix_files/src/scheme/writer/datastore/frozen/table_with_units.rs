use crate::common::serialize::{serialize_shareable_string, serialize_shareable_strings};
use crate::scheme::writer::datastore::definition::common::table::serialize_table_rows;
use crate::scheme::writer::datastore::definition::table_with_units::TableWithUnitsDefinitionWriterV1;
use hrafnix_datastore::frozen::TableWithUnitsFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen table definition, cell expressions, and column units.
#[derive(Debug, Serialize)]
pub(crate) struct TableWithUnitsWriterV1 {
    /// Table metadata, ordered unit-aware columns, and default rows.
    definition: TableWithUnitsDefinitionWriterV1,
    /// Unevaluated table reference, or an empty string for local rows.
    #[serde(serialize_with = "serialize_shareable_string")]
    parameter: ShareableString,
    /// Selected unit strings in definition column order.
    #[serde(serialize_with = "serialize_shareable_strings")]
    units: Vec<ShareableString>,
    /// Row-major cell expressions in definition column order.
    #[serde(serialize_with = "serialize_table_rows")]
    rows: Vec<Vec<ShareableString>>,
}

impl From<&TableWithUnitsFrozen> for TableWithUnitsWriterV1 {
    fn from(table: &TableWithUnitsFrozen) -> Self {
        Self {
            definition: table.definition().into(),
            parameter: table.parameter().clone(),
            units: table.units().to_vec(),
            rows: table.rows().to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::{
        NumberConstraint, NumberWithUnitsDefinition, TableColumnRequirement,
        TableWithUnitsDefinition,
    };
    use hrafnix_keys::StoreKey;
    use hrafnix_units::UnitId;
    use serde_json::json;

    #[test]
    fn writes_rows_column_units_and_explicit_parameter() {
        let definition = TableWithUnitsDefinition::new_with_column_requirements_and_default(
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
        for parameter in ["", "p_source"] {
            let table = TableWithUnitsFrozen::new_from_rows_with_parameter(
                definition.clone(),
                vec![
                    vec!["2 * p_length".into(), "3".into()],
                    vec!["4 m".into(), "5".into()],
                ],
                vec!["length_centimeter".into(), "none".into()],
                parameter,
            );

            assert_eq!(
                serde_json::to_value(TableWithUnitsWriterV1::from(&table)).unwrap(),
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
                    "parameter": parameter,
                    "units": ["length_centimeter", "none"],
                    "rows": [["2 * p_length", "3"], ["4 m", "5"]]
                })
            );
        }
    }
}
