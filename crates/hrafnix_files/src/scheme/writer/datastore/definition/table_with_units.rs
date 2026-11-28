use super::common::table::{RequirementDefinitionWriterV1, serialize_optional_table_rows};
use super::number_with_units::NumberWithUnitDefinitionWriterV1;
use crate::common::serialize::{serialize_shareable_string, serialize_store_key};
use hrafnix_datastore::definition::{TableColumnRequirement, TableWithUnitsDefinition};
use hrafnix_keys::StoreKey;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// One version-1 numeric column with units.
#[derive(Debug, Serialize)]
pub(crate) struct TableWithUnitsColumnDefinitionWriterV1 {
    /// Plain-string column identifier.
    #[serde(serialize_with = "serialize_store_key")]
    key: StoreKey,
    /// Inline numeric definition with units.
    #[serde(flatten)]
    definition: NumberWithUnitDefinitionWriterV1,
    /// Adjacent-row requirement.
    #[serde(with = "RequirementDefinitionWriterV1")]
    requirement: TableColumnRequirement,
}

/// Version-1 numeric table definition with units.
#[derive(Debug, Serialize)]
pub(crate) struct TableWithUnitsDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Ordered columns.
    columns: Vec<TableWithUnitsColumnDefinitionWriterV1>,
    /// Required nullable default rows.
    #[serde(serialize_with = "serialize_optional_table_rows")]
    default: Option<Vec<Vec<ShareableString>>>,
}

impl From<&TableWithUnitsDefinition> for TableWithUnitsDefinitionWriterV1 {
    fn from(table: &TableWithUnitsDefinition) -> Self {
        Self {
            description: table.description(),
            columns: table
                .iter()
                .zip(table.column_requirements())
                .map(|((key, definition), (_, requirement))| {
                    TableWithUnitsColumnDefinitionWriterV1 {
                        key: key.clone(),
                        definition: definition.into(),
                        requirement,
                    }
                })
                .collect(),
            default: table.default_table().map(<[_]>::to_vec),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::{NumberConstraint, NumberWithUnitsDefinition};
    use hrafnix_units::UnitId;
    use serde_json::{Value, json};

    fn columns() -> Vec<(StoreKey, NumberWithUnitsDefinition, TableColumnRequirement)> {
        vec![
            (
                StoreKey::new("z".into()).unwrap(),
                NumberWithUnitsDefinition::new_with_constraint_and_default(
                    "Distance",
                    NumberConstraint::min(0.0, true),
                    "1 m",
                    UnitId::Length_Meter,
                ),
                TableColumnRequirement::Decreasing,
            ),
            (
                StoreKey::new("a".into()).unwrap(),
                NumberWithUnitsDefinition::new_with_default("Ratio", "4", UnitId::None),
                TableColumnRequirement::None,
            ),
        ]
    }

    fn expected(default: &Value) -> Value {
        json!({
            "description": "Positions",
            "columns": [
                {
                    "key": "z", "description": "Distance", "default": "1 m",
                    "constraint": {"type": "min", "min": 0.0, "inclusive": true},
                    "requirement": "decreasing", "preferred_unit": "length_meter"
                },
                {
                    "key": "a", "description": "Ratio", "default": "4",
                    "constraint": {"type": "none"}, "requirement": "none",
                    "preferred_unit": "none"
                }
            ],
            "default": default
        })
    }

    #[test]
    fn writes_ordered_columns_units_requirements_and_rows() {
        let definition = TableWithUnitsDefinition::new_with_column_requirements_and_default(
            "Positions",
            columns(),
            vec![vec!["3 m", "6"], vec!["2 m", "8"]],
        );
        assert_eq!(
            serde_json::to_value(TableWithUnitsDefinitionWriterV1::from(&definition)).unwrap(),
            expected(&json!([["3 m", "6"], ["2 m", "8"]]))
        );
    }

    #[test]
    fn preserves_null_and_explicitly_empty_defaults() {
        let absent = TableWithUnitsDefinition::new_with_column_requirements("Positions", columns());
        let empty = TableWithUnitsDefinition::new_with_column_requirements_and_default(
            "Positions",
            columns(),
            Vec::<Vec<String>>::new(),
        );
        assert_eq!(
            serde_json::to_value(TableWithUnitsDefinitionWriterV1::from(&absent)).unwrap(),
            expected(&Value::Null)
        );
        assert_eq!(
            serde_json::to_value(TableWithUnitsDefinitionWriterV1::from(&empty)).unwrap(),
            expected(&json!([]))
        );
    }
}
