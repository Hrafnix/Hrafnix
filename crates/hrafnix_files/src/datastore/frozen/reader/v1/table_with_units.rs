use crate::common::deserialize::deserialize_object;
use crate::common::validate::validate_unit_count;
use crate::datastore::definition::reader::v1::table_with_units::TableWithUnitsDefinitionReaderV1;
use hrafnix_datastore::frozen::TableWithUnitsFrozen;
use hrafnix_message::message::Message;
use hrafnix_shareable_string::ShareableString;
use serde::Deserialize;

/// Version-1 frozen table definition, cell expressions, and column units.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
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

impl TryFrom<TableWithUnitsReaderV1> for TableWithUnitsFrozen {
    type Error = Message;

    fn try_from(table: TableWithUnitsReaderV1) -> Result<Self, Self::Error> {
        validate_unit_count(table.units.len(), table.definition.column_count())?;
        let units = table.units.into_iter().map(ShareableString::from).collect();
        let rows = table
            .rows
            .into_iter()
            .map(|row| row.into_iter().map(ShareableString::from).collect())
            .collect();
        Ok(TableWithUnitsFrozen::new_from_rows_with_parameter(
            table.definition.try_into()?,
            rows,
            units,
            table.parameter,
        ))
    }
}
