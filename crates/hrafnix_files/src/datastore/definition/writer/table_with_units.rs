use super::common::table::{RequirementWriterV1, serialize_optional_table_rows};
use super::number_with_units::NumberWithUnitWriterV1;
use crate::common::serialize::{serialize_shareable_string, serialize_store_key};
use hrafnix_datastore::definition::{TableColumnRequirement, TableWithUnitsDefinition};
use hrafnix_keys::StoreKey;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// One version-1 numeric column with units.
#[derive(Debug, Serialize)]
pub(crate) struct TableWithUnitsColumnWriterV1 {
    /// Plain-string column identifier.
    #[serde(serialize_with = "serialize_store_key")]
    key: StoreKey,
    /// Inline numeric definition with units.
    #[serde(flatten)]
    definition: NumberWithUnitWriterV1,
    /// Adjacent-row requirement.
    #[serde(with = "RequirementWriterV1")]
    requirement: TableColumnRequirement,
}

/// Version-1 numeric table definition with units.
#[derive(Debug, Serialize)]
pub(crate) struct TableWithUnitsWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Ordered columns.
    columns: Vec<TableWithUnitsColumnWriterV1>,
    /// Required nullable default rows.
    #[serde(serialize_with = "serialize_optional_table_rows")]
    default: Option<Vec<Vec<ShareableString>>>,
}

impl From<&TableWithUnitsDefinition> for TableWithUnitsWriterV1 {
    fn from(table: &TableWithUnitsDefinition) -> Self {
        Self {
            description: table.description(),
            columns: table
                .iter()
                .zip(table.column_requirements())
                .map(
                    |((key, definition), (_, requirement))| TableWithUnitsColumnWriterV1 {
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
