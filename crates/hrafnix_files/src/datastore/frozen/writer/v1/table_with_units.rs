use crate::common::serialize::serialize_shareable_strings;
use crate::datastore::definition::writer::common::table::serialize_table_rows;
use crate::datastore::definition::writer::table_with_units::TableWithUnitsDefinitionWriterV1;
use hrafnix_datastore::frozen::TableWithUnitsFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen table definition, cell expressions, and column units.
#[derive(Debug, Serialize)]
pub(crate) struct TableWithUnitsWriterV1 {
    /// Table metadata, ordered unit-aware columns, and default rows.
    definition: TableWithUnitsDefinitionWriterV1,
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
            units: table.units().to_vec(),
            rows: table.rows().to_vec(),
        }
    }
}
