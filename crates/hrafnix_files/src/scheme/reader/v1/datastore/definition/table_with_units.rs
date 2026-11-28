use crate::common::deserialize::{deserialize_objects, deserialize_unit_variant};
use crate::common::validate::{ensure_unique_keys, validate_default_rows};
use crate::scheme::reader::v1::datastore::definition::common::constraints::ConstraintNumberDefinitionReaderV1;
use crate::scheme::reader::v1::datastore::definition::common::table::RequirementDefinitionReaderV1;
use crate::scheme::reader::v1::datastore::definition::common::units::UnitIdEnumReaderV1;
use hrafnix_datastore::definition::{NumberWithUnitsDefinition, TableWithUnitsDefinition};
use hrafnix_keys::StoreKey;
use hrafnix_message::message::Message;
use serde::Deserialize;

/// One version-1 numeric column with units.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TableWithUnitsColumnDefinitionReaderV1 {
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
    /// Preferred display unit.
    preferred_unit: UnitIdEnumReaderV1,
}

/// Version-1 numeric table definition with units.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TableWithUnitsDefinitionReaderV1 {
    /// Item description.
    description: String,
    /// Ordered columns; keys are checked for uniqueness during conversion.
    #[serde(deserialize_with = "deserialize_objects")]
    columns: Vec<TableWithUnitsColumnDefinitionReaderV1>,
    /// Required nullable default rows, each with one value per column.
    #[serde(deserialize_with = "Option::deserialize")]
    default: Option<Vec<Vec<String>>>,
}

impl TableWithUnitsDefinitionReaderV1 {
    /// Returns the number of columns.
    pub(crate) const fn column_count(&self) -> usize {
        self.columns.len()
    }
}

impl TryFrom<TableWithUnitsDefinitionReaderV1> for TableWithUnitsDefinition {
    type Error = Message;

    fn try_from(table: TableWithUnitsDefinitionReaderV1) -> Result<Self, Self::Error> {
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
                    NumberWithUnitsDefinition::new_with_constraint_and_default(
                        column.description,
                        column.constraint.into(),
                        column.default,
                        column.preferred_unit.into(),
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
    use hrafnix_units::UnitId;
    use serde_json::{Value, json};

    fn table_json(default: &Value) -> Value {
        json!({
            "description": "Table with units",
            "columns": [
                {
                    "key": "z",
                    "description": "Distance",
                    "default": "1 + 2",
                    "constraint": {"type": "min", "min": 0.0, "inclusive": true},
                    "requirement": "increasing",
                    "preferred_unit": "length_meter"
                },
                {
                    "key": "a",
                    "description": "Ratio",
                    "default": "4",
                    "constraint": {"type": "none"},
                    "requirement": "none",
                    "preferred_unit": "none"
                }
            ],
            "default": default
        })
    }

    fn read_table(value: Value) -> TableWithUnitsDefinition {
        try_read(value).expect("valid table definition")
    }

    fn try_read(value: Value) -> Result<TableWithUnitsDefinition, ReadError> {
        read_owned_value::<TableWithUnitsDefinitionReaderV1, _>(value)
    }

    #[test]
    fn preserves_columns_units_and_default_rows() {
        let table = read_table(table_json(&json!([["5 m", "6"], ["7 m", "8"]])));
        let expected = TableWithUnitsDefinition::new_with_column_requirements_and_default(
            "Table with units",
            vec![
                (
                    StoreKey::new("z".into()).expect("valid key"),
                    NumberWithUnitsDefinition::new_with_constraint_and_default(
                        "Distance",
                        NumberConstraint::min(0.0, true),
                        "1 + 2",
                        UnitId::Length_Meter,
                    ),
                    TableColumnRequirement::Increasing,
                ),
                (
                    StoreKey::new("a".into()).expect("valid key"),
                    NumberWithUnitsDefinition::new_with_default("Ratio", "4", UnitId::None),
                    TableColumnRequirement::None,
                ),
            ],
            vec![vec!["5 m", "6"], vec!["7 m", "8"]],
        );
        assert_eq!(table, expected);
    }

    #[test]
    fn distinguishes_null_and_empty_defaults() {
        let table = read_table(table_json(&Value::Null));
        assert_eq!(table.default_table(), None);
        assert_eq!(
            table.column_requirement("z"),
            Some(TableColumnRequirement::Increasing)
        );
        assert_eq!(
            read_table(table_json(&json!([]))).default_table(),
            Some([].as_slice())
        );
    }

    #[test]
    fn accepts_empty_columns() {
        for default in [Value::Null, json!([])] {
            let table = read_table(json!({
                "description": "",
                "columns": [],
                "default": default
            }));
            assert_eq!(table.count(), 0);
        }
    }

    #[test]
    fn requires_explicit_fields() {
        for field in ["description", "columns", "default"] {
            let mut value = table_json(&Value::Null);
            value.as_object_mut().expect("table object").remove(field);
            let error = try_read(value).expect_err("missing table field must be rejected");
            assert!(
                error
                    .to_string()
                    .contains(&format!("missing field `{field}`"))
            );
        }

        for field in [
            "key",
            "description",
            "default",
            "constraint",
            "requirement",
            "preferred_unit",
        ] {
            let mut value = table_json(&Value::Null);
            value["columns"][0]
                .as_object_mut()
                .expect("column object")
                .remove(field);
            let error = try_read(value).expect_err("missing column field must be rejected");
            assert!(
                error
                    .to_string()
                    .contains(&format!("missing field `{field}`"))
            );
        }
    }

    #[test]
    fn rejects_duplicate_column_keys_and_wrong_row_widths() {
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

        let error = try_read(table_json(&json!([["1 m", "2"], ["3 m"]])))
            .expect_err("mismatched row must be rejected")
            .message();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_table_default_row_width"
        );
        assert_eq!(error.translate_data().message_params()["row"].as_str(), "1");
    }

    #[test]
    fn rejects_invalid_columns() {
        for (field, invalid) in [
            ("key", json!("invalid key")),
            ("preferred_unit", json!("unknown")),
            ("preferred_unit", json!({"length_meter": null})),
            ("preferred_unit", Value::Null),
            ("requirement", json!("unknown")),
            ("requirement", json!({"increasing": null})),
            ("requirement", Value::Null),
            ("unexpected", json!(true)),
        ] {
            let mut value = table_json(&Value::Null);
            value["columns"][0][field] = invalid;
            assert!(try_read(value).is_err(), "{field}");
        }

        let mut value = table_json(&Value::Null);
        value["columns"][0] =
            json!(["z", "Distance", "0", {"type": "none"}, "none", "length_meter"]);
        assert!(try_read(value).is_err());
    }

    #[test]
    fn rejects_invalid_tables() {
        let mut value = table_json(&Value::Null);
        value["unexpected"] = json!(true);
        assert!(try_read(value).is_err());

        for default in [json!([["1 m", 2]]), json!(["1 m"]), json!({})] {
            assert!(try_read(table_json(&default)).is_err());
        }

        let value = table_json(&Value::Null);
        let positional = json!([value["description"], value["columns"], value["default"]]);
        assert!(try_read(positional).is_err());
    }
}
