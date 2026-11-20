use crate::common::deserialize::deserialize_object;
use crate::datastore::definition::reader::v1::table::TableDefinitionReaderV1;
use hrafnix_datastore::frozen::TableFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Deserialize;

/// Version-1 frozen table definition and unevaluated cell expressions.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TableReaderV1 {
    /// Table metadata, ordered columns, and default rows.
    #[serde(deserialize_with = "deserialize_object")]
    definition: TableDefinitionReaderV1,
    /// Row-major cell expressions in definition column order.
    rows: Vec<Vec<String>>,
}

impl From<TableReaderV1> for TableFrozen {
    fn from(table: TableReaderV1) -> Self {
        let rows = table
            .rows
            .into_iter()
            .map(|row| row.into_iter().map(ShareableString::from).collect())
            .collect();
        TableFrozen::new_from_rows(table.definition.into(), rows)
    }
}
