use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::table::TableDefinitionReaderV1;
use hrafnix_datastore::frozen::TableFrozen;
use hrafnix_message::message::Message;
use hrafnix_shareable_string::ShareableString;
use serde::Deserialize;

/// Version-1 frozen table definition and unevaluated cell expressions.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TableReaderV1 {
    /// Table metadata, ordered columns, and default rows.
    #[serde(deserialize_with = "deserialize_object")]
    definition: TableDefinitionReaderV1,
    /// Unevaluated table reference, or an empty string for local rows.
    parameter: String,
    /// Row-major cell expressions in definition column order.
    rows: Vec<Vec<String>>,
}

impl TryFrom<TableReaderV1> for TableFrozen {
    type Error = Message;

    fn try_from(table: TableReaderV1) -> Result<Self, Self::Error> {
        let rows = table
            .rows
            .into_iter()
            .map(|row| row.into_iter().map(ShareableString::from).collect())
            .collect();
        Ok(TableFrozen::new_from_rows_with_parameter(
            table.definition.try_into()?,
            rows,
            table.parameter,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::{
        NumberConstraint, NumberDefinition, TableColumnRequirement,
    };
    use serde_json::{Value, json};

    fn document() -> Value {
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
            "parameter": "p_source",
            "rows": [["2", "3 + 4"], ["5", "6"]]
        })
    }

    #[test]
    fn reads_definition_reference_and_nonempty_rows_in_column_order() {
        let reader = serde_json::from_value::<RootObject<TableReaderV1>>(document()).unwrap();
        let table = TableFrozen::try_from(reader.document).unwrap();
        let expected_definition = hrafnix_datastore::definition::TableDefinition::new_with_column_requirements_and_default(
                "Samples",
                vec![
                    (
                        hrafnix_keys::StoreKey::new("z".into()).unwrap(),
                        NumberDefinition::new_with_constraint_and_default(
                            "Time", NumberConstraint::min(0.0, true), "0"
                        ),
                        TableColumnRequirement::Increasing,
                    ),
                    (
                        hrafnix_keys::StoreKey::new("a".into()).unwrap(),
                        NumberDefinition::new_with_default("Value", "1"),
                        TableColumnRequirement::None,
                    ),
                ],
                vec![vec!["0", "1"]],
            );

        assert_eq!(table.definition(), &expected_definition);
        assert_eq!(table.parameter(), "p_source");
        assert_eq!(table.row_count(), 2);
        assert_eq!(
            table.rows(),
            &[
                vec![ShareableString::from("2"), "3 + 4".into()],
                vec!["5".into(), "6".into()]
            ]
        );
        assert_eq!(table.cell_by_name(0, "a").unwrap(), "3 + 4");
    }

    #[test]
    fn requires_explicit_parameter_even_for_local_rows() {
        let mut value = document();
        value.as_object_mut().unwrap().remove("parameter");
        assert!(serde_json::from_value::<RootObject<TableReaderV1>>(value).is_err());
    }

    #[test]
    fn propagates_duplicate_column_validation() {
        let mut value = document();
        value["definition"]["columns"][1]["key"] = json!("z");
        let reader = serde_json::from_value::<RootObject<TableReaderV1>>(value).unwrap();
        let error = TableFrozen::try_from(reader.document).unwrap_err();

        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_duplicate_column_key"
        );
    }
}
