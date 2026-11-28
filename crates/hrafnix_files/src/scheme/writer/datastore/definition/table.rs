use super::common::table::{RequirementDefinitionWriterV1, serialize_optional_table_rows};
use super::number::NumberDefinitionWriterV1;
use crate::common::serialize::{serialize_shareable_string, serialize_store_key};
use hrafnix_datastore::definition::{TableColumnRequirement, TableDefinition};
use hrafnix_keys::StoreKey;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// One version-1 numeric column.
#[derive(Debug, Serialize)]
pub(crate) struct TableColumnDefinitionWriterV1 {
    /// Plain-string column identifier.
    #[serde(serialize_with = "serialize_store_key")]
    key: StoreKey,
    /// Inline numeric definition.
    #[serde(flatten)]
    definition: NumberDefinitionWriterV1,
    /// Adjacent-row requirement.
    #[serde(with = "RequirementDefinitionWriterV1")]
    requirement: TableColumnRequirement,
}

/// Version-1 numeric table definition.
#[derive(Debug, Serialize)]
pub(crate) struct TableDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Ordered columns.
    columns: Vec<TableColumnDefinitionWriterV1>,
    /// Required nullable default rows.
    #[serde(serialize_with = "serialize_optional_table_rows")]
    default: Option<Vec<Vec<ShareableString>>>,
}

impl From<&TableDefinition> for TableDefinitionWriterV1 {
    fn from(table: &TableDefinition) -> Self {
        Self {
            description: table.description(),
            columns: table
                .iter()
                .zip(table.column_requirements())
                .map(
                    |((key, definition), (_, requirement))| TableColumnDefinitionWriterV1 {
                        key: key.clone(),
                        definition: definition.into(),
                        requirement,
                    },
                )
                .collect(),
            default: table.default_table().map(<[_]>::to_vec),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::{NumberConstraint, NumberDefinition};
    use serde_json::{Value, json};

    fn columns() -> Vec<(StoreKey, NumberDefinition, TableColumnRequirement)> {
        vec![
            (
                StoreKey::new("z".into()).unwrap(),
                NumberDefinition::new_with_constraint_and_default(
                    "First",
                    NumberConstraint::min(-1.5, false),
                    "1 + 2",
                ),
                TableColumnRequirement::Increasing,
            ),
            (
                StoreKey::new("a".into()).unwrap(),
                NumberDefinition::new_with_default("Second", "4"),
                TableColumnRequirement::None,
            ),
        ]
    }

    fn expected(default: &Value) -> Value {
        json!({
            "description": "Samples",
            "columns": [
                {
                    "key": "z", "description": "First", "default": "1 + 2",
                    "constraint": {"type": "min", "min": -1.5, "inclusive": false},
                    "requirement": "increasing"
                },
                {
                    "key": "a", "description": "Second", "default": "4",
                    "constraint": {"type": "none"}, "requirement": "none"
                }
            ],
            "default": default
        })
    }

    #[test]
    fn writes_ordered_inline_columns_requirements_and_default_rows() {
        let definition = TableDefinition::new_with_column_requirements_and_default(
            "Samples",
            columns(),
            vec![vec!["5", "6"], vec!["7", "8"]],
        );
        assert_eq!(
            serde_json::to_value(TableDefinitionWriterV1::from(&definition)).unwrap(),
            expected(&json!([["5", "6"], ["7", "8"]]))
        );
    }

    #[test]
    fn preserves_null_and_explicitly_empty_defaults() {
        let absent = TableDefinition::new_with_column_requirements("Samples", columns());
        let empty = TableDefinition::new_with_column_requirements_and_default(
            "Samples",
            columns(),
            Vec::<Vec<String>>::new(),
        );
        assert_eq!(
            serde_json::to_value(TableDefinitionWriterV1::from(&absent)).unwrap(),
            expected(&Value::Null)
        );
        assert_eq!(
            serde_json::to_value(TableDefinitionWriterV1::from(&empty)).unwrap(),
            expected(&json!([]))
        );
    }
}
