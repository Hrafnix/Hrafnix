use crate::common::serialize::serialize_shareable_string;
use crate::scheme::writer::datastore::definition::common::table::serialize_table_rows;
use crate::scheme::writer::datastore::definition::table::TableDefinitionWriterV1;
use hrafnix_datastore::frozen::TableFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen table definition and unevaluated cell expressions.
#[derive(Debug, Serialize)]
pub(crate) struct TableWriterV1 {
    /// Table metadata, ordered columns, and default rows.
    definition: TableDefinitionWriterV1,
    /// Unevaluated table reference, or an empty string for local rows.
    #[serde(serialize_with = "serialize_shareable_string")]
    parameter: ShareableString,
    /// Row-major cell expressions in definition column order.
    #[serde(serialize_with = "serialize_table_rows")]
    rows: Vec<Vec<ShareableString>>,
}

impl From<&TableFrozen> for TableWriterV1 {
    fn from(table: &TableFrozen) -> Self {
        Self {
            definition: table.definition().into(),
            parameter: table.parameter().clone(),
            rows: table.rows().to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::{
        NumberConstraint, NumberDefinition, TableColumnRequirement, TableDefinition,
    };
    use hrafnix_keys::StoreKey;
    use serde_json::json;

    #[test]
    fn writes_ordered_columns_nonempty_rows_and_explicit_parameter() {
        let definition = TableDefinition::new_with_column_requirements_and_default(
            "Samples",
            vec![
                (
                    StoreKey::new("z".into()).unwrap(),
                    NumberDefinition::new_with_constraint_and_default(
                        "Time",
                        NumberConstraint::min(0.0, true),
                        "0",
                    ),
                    TableColumnRequirement::Increasing,
                ),
                (
                    StoreKey::new("a".into()).unwrap(),
                    NumberDefinition::new_with_default("Value", "1"),
                    TableColumnRequirement::None,
                ),
            ],
            vec![vec!["0", "1"]],
        );
        for parameter in ["", "p_source"] {
            let table = TableFrozen::new_from_rows_with_parameter(
                definition.clone(),
                vec![
                    vec!["2".into(), "3 + 4".into()],
                    vec!["5".into(), "6".into()],
                ],
                parameter,
            );

            assert_eq!(
                serde_json::to_value(TableWriterV1::from(&table)).unwrap(),
                json!({
                    "definition": {
                        "description": "Samples",
                        "columns": [
                            {
                                "key": "z", "description": "Time", "default": "0",
                                "constraint": {"type": "min", "min": 0.0, "inclusive": true},
                                "requirement": "increasing"
                            },
                            {
                                "key": "a", "description": "Value", "default": "1",
                                "constraint": {"type": "none"}, "requirement": "none"
                            }
                        ],
                        "default": [["0", "1"]]
                    },
                    "parameter": parameter,
                    "rows": [["2", "3 + 4"], ["5", "6"]]
                })
            );
        }
    }
}
