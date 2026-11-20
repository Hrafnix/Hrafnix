use crate::common::deserialize::deserialize_object;
use crate::datastore::definition::reader::v1::table_with_units::TableWithUnitsDefinitionReaderV1;
use hrafnix_datastore::frozen::TableWithUnitsFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Deserialize;

/// Version-1 frozen table definition, cell expressions, and column units.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TableWithUnitsReaderV1 {
    /// Table metadata, ordered unit-aware columns, and default rows.
    #[serde(deserialize_with = "deserialize_object")]
    definition: TableWithUnitsDefinitionReaderV1,
    /// Selected unit strings in definition column order.
    units: Vec<String>,
    /// Row-major cell expressions in definition column order.
    rows: Vec<Vec<String>>,
}

impl From<TableWithUnitsReaderV1> for TableWithUnitsFrozen {
    fn from(table: TableWithUnitsReaderV1) -> Self {
        let units = table.units.into_iter().map(ShareableString::from).collect();
        let rows = table
            .rows
            .into_iter()
            .map(|row| row.into_iter().map(ShareableString::from).collect())
            .collect();
        TableWithUnitsFrozen::new_from_rows(table.definition.into(), rows, units)
    }
}
