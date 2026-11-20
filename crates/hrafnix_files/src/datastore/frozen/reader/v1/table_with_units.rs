use crate::common::deserialize::deserialize_object;
use crate::datastore::definition::reader::v1::table_with_units::TableWithUnitsDefinitionReaderV1;
use hrafnix_datastore::frozen::TableWithUnitsFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::{Deserialize, Deserializer, de::Error};

/// Version-1 frozen table definition, cell expressions, and column units.
#[derive(Debug, Deserialize)]
#[serde(remote = "Self", deny_unknown_fields)]
pub(crate) struct TableWithUnitsReaderV1 {
    /// Table metadata, ordered unit-aware columns, and default rows.
    #[serde(deserialize_with = "deserialize_object")]
    definition: TableWithUnitsDefinitionReaderV1,
    /// Unevaluated table reference, or an empty string for local rows.
    parameter: String,
    /// Selected unit strings in definition column order.
    units: Vec<String>,
    /// Row-major cell expressions in definition column order.
    rows: Vec<Vec<String>>,
}

impl<'de> Deserialize<'de> for TableWithUnitsReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let table = Self::deserialize(deserializer)?;
        let column_count = table.definition.column_count();
        if table.units.len() != column_count {
            return Err(D::Error::custom(format!(
                "table has {} units but {column_count} columns",
                table.units.len()
            )));
        }
        Ok(table)
    }
}

impl From<TableWithUnitsReaderV1> for TableWithUnitsFrozen {
    fn from(table: TableWithUnitsReaderV1) -> Self {
        let units = table.units.into_iter().map(ShareableString::from).collect();
        let rows = table
            .rows
            .into_iter()
            .map(|row| row.into_iter().map(ShareableString::from).collect())
            .collect();
        TableWithUnitsFrozen::new_from_rows_with_parameter(
            table.definition.into(),
            rows,
            units,
            table.parameter,
        )
    }
}
