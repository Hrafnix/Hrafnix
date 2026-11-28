use crate::common::deserialize::{deserialize_objects, deserialize_unit_variant};
use crate::common::validate::{ensure_unique_keys, validate_default_rows};
use crate::scheme::reader::v1::datastore::definition::common::constraints::ConstraintNumberDefinitionReaderV1;
use crate::scheme::reader::v1::datastore::definition::common::table::RequirementDefinitionReaderV1;
use hrafnix_datastore::definition::{NumberDefinition, TableDefinition};
use hrafnix_keys::StoreKey;
use hrafnix_message::message::Message;
use serde::Deserialize;

/// One version 1 numeric column.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TableColumnDefinitionReaderV1 {
    /// Plain-string column identifier, validated during conversion.
    key: String,
    /// Column description.
    description: String,
    /// Default cell expression.
    default: String,
    /// Numeric bounds.
    constraint: ConstraintNumberDefinitionReaderV1,
    /// Adjacent-row requirement.
    #[serde(deserialize_with = "deserialize_unit_variant")]
    requirement: RequirementDefinitionReaderV1,
}

/// Version-1 numeric table definition.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TableDefinitionReaderV1 {
    /// Item description.
    description: String,
    /// Ordered columns; keys are checked for uniqueness during conversion.
    #[serde(deserialize_with = "deserialize_objects")]
    columns: Vec<TableColumnDefinitionReaderV1>,
    /// Required nullable default rows, each with one value per column.
    #[serde(deserialize_with = "Option::deserialize")]
    default: Option<Vec<Vec<String>>>,
}

impl TableDefinitionReaderV1 {
    /// Returns the number of columns.
    pub(crate) const fn column_count(&self) -> usize {
        self.columns.len()
    }
}

impl TryFrom<TableDefinitionReaderV1> for TableDefinition {
    type Error = Message;

    fn try_from(table: TableDefinitionReaderV1) -> Result<Self, Self::Error> {
        ensure_unique_keys(
            table.columns.iter().map(|column| &column.key),
            "files_duplicate_column_key",
        )?;
        if let Some(rows) = &table.default {
            validate_default_rows(rows, table.column_count())?;
        }
        let columns = table
            .columns
            .into_iter()
            .map(|column| {
                Ok((
                    StoreKey::new(column.key.into())?,
                    NumberDefinition::new_with_constraint_and_default(
                        column.description,
                        column.constraint.into(),
                        column.default,
                    ),
                    column.requirement.into(),
                ))
            })
            .collect::<Result<_, Message>>()?;
        Ok(match table.default {
            Some(default) => {
                Self::new_with_column_requirements_and_default(table.description, columns, default)
            }
            None => Self::new_with_column_requirements(table.description, columns),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_support::{ReadError, read_owned_value};
    use hrafnix_datastore::definition::{NumberConstraint, TableColumnRequirement};
    use serde_json::{Value, json};

    fn table_json(default: &Value) -> Value {
        json!({
            "description": "Numeric table",
            "columns": [
                {
                    "key": "z",
                    "description": "First column",
                    "default": "1 + 2",
                    "constraint": {"type": "min", "min": 0.0, "inclusive": true},
                    "requirement": "increasing"
                },
                {
                    "key": "a",
                    "description": "Second column",
                    "default": "4",
                    "constraint": {"type": "none"},
                    "requirement": "none"
                }
            ],
            "default": default
        })
    }

    fn read_table(value: Value) -> TableDefinition {
        try_read(value).expect("valid table definition")
    }

    fn try_read(value: Value) -> Result<TableDefinition, ReadError> {
        read_owned_value::<TableDefinitionReaderV1, _>(value)
    }

    #[test]
    fn preserves_columns_and_default_rows() {
        let table = read_table(table_json(&json!([["5", "6"], ["7", "8"]])));
        let expected = TableDefinition::new_with_column_requirements_and_default(
            "Numeric table",
            vec![
                (
                    StoreKey::new("z".into()).expect("valid key"),
                    NumberDefinition::new_with_constraint_and_default(
                        "First column",
                        NumberConstraint::min(0.0, true),
                        "1 + 2",
                    ),
                    TableColumnRequirement::Increasing,
                ),
                (
                    StoreKey::new("a".into()).expect("valid key"),
                    NumberDefinition::new_with_default("Second column", "4"),
                    TableColumnRequirement::None,
                ),
            ],
            vec![vec!["5", "6"], vec!["7", "8"]],
        );
        assert_eq!(table, expected);
    }

    #[test]
    fn distinguishes_null_and_empty_defaults() {
        assert_eq!(read_table(table_json(&Value::Null)).default_table(), None);
        assert_eq!(
            read_table(table_json(&json!([]))).default_table(),
            Some([].as_slice())
        );
    }

    #[test]
    fn converts_all_column_requirements() {
        for (name, expected) in [
            ("none", TableColumnRequirement::None),
            (
                "constant_or_increasing",
                TableColumnRequirement::ConstantOrIncreasing,
            ),
            ("increasing", TableColumnRequirement::Increasing),
            (
                "constant_or_decreasing",
                TableColumnRequirement::ConstantOrDecreasing,
            ),
            ("decreasing", TableColumnRequirement::Decreasing),
            ("constant", TableColumnRequirement::Constant),
        ] {
            let mut value = table_json(&Value::Null);
            value["columns"][0]["requirement"] = json!(name);
            let table = read_table(value);
            assert_eq!(table.column_requirement("z"), Some(expected), "{name}");
        }
    }

    #[test]
    fn requires_explicit_default() {
        let mut value = table_json(&Value::Null);
        value
            .as_object_mut()
            .expect("table object")
            .remove("default");
        let error = try_read(value).expect_err("missing default must be rejected");
        assert!(error.to_string().contains("missing field `default`"));
    }

    #[test]
    fn rejects_duplicate_column_keys() {
        let mut value = table_json(&Value::Null);
        value["columns"][1]["key"] = json!("z");
        let error = try_read(value)
            .expect_err("duplicate column key must be rejected")
            .message();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_duplicate_column_key"
        );
        assert_eq!(error.translate_data().message_params()["key"].as_str(), "z");
    }

    #[test]
    fn rejects_default_rows_with_wrong_width() {
        for (rows, row, values) in [
            (json!([["1", "2", "3"]]), "0", "3"),
            (json!([["1", "2"], ["3"]]), "1", "1"),
            (json!([[]]), "0", "0"),
        ] {
            let error = try_read(table_json(&rows))
                .expect_err("mismatched row must be rejected")
                .message();
            let data = error.translate_data();
            assert_eq!(data.message_key().as_str(), "files_table_default_row_width");
            assert_eq!(data.message_params()["row"].as_str(), row, "{rows}");
            assert_eq!(data.message_params()["values"].as_str(), values, "{rows}");
            assert_eq!(data.message_params()["columns"].as_str(), "2", "{rows}");
        }
    }

    #[test]
    fn rejects_invalid_columns() {
        for column in [
            json!(["z", "First column", "0", {"type": "none"}, "none"]),
            json!({
                "key": "invalid key",
                "description": "",
                "default": "0",
                "constraint": {"type": "none"},
                "requirement": "none"
            }),
        ] {
            let mut value = table_json(&Value::Null);
            value["columns"][0] = column;
            assert!(try_read(value).is_err());
        }

        for requirement in [json!({"increasing": null}), json!("unknown"), Value::Null] {
            let mut value = table_json(&Value::Null);
            value["columns"][0]["requirement"] = requirement;
            assert!(try_read(value).is_err());
        }
    }
}
