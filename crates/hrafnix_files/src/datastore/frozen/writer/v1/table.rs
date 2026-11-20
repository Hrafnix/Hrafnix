use crate::datastore::definition::writer::common::table::serialize_table_rows;
use crate::datastore::definition::writer::table::TableDefinitionWriterV1;
use hrafnix_datastore::frozen::TableFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen table definition and unevaluated cell expressions.
#[derive(Debug, Serialize)]
pub(crate) struct TableWriterV1 {
    /// Table metadata, ordered columns, and default rows.
    definition: TableDefinitionWriterV1,
    /// Row-major cell expressions in definition column order.
    #[serde(serialize_with = "serialize_table_rows")]
    rows: Vec<Vec<ShareableString>>,
}

impl From<&TableFrozen> for TableWriterV1 {
    fn from(table: &TableFrozen) -> Self {
        Self {
            definition: table.definition().into(),
            rows: table.rows().to_vec(),
        }
    }
}
