use crate::common::deserialize::deserialize_object;
use crate::datastore::definition::reader::v1::table::TableDefinitionReaderV1;
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
