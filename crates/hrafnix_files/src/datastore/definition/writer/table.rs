use super::common::table::{RequirementWriterV1, serialize_optional_table_rows};
use super::number::NumberWriterV1;
use crate::common::serialize::{serialize_shareable_string, serialize_store_key};
use hrafnix_datastore::definition::{TableColumnRequirement, TableDefinition};
use hrafnix_keys::StoreKey;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// One version-1 numeric column.
#[derive(Debug, Serialize)]
pub(crate) struct TableColumnWriterV1 {
    /// Plain-string column identifier.
    #[serde(serialize_with = "serialize_store_key")]
    key: StoreKey,
    /// Inline numeric definition.
    #[serde(flatten)]
    definition: NumberWriterV1,
    /// Adjacent-row requirement.
    #[serde(with = "RequirementWriterV1")]
    requirement: TableColumnRequirement,
}

/// Version-1 numeric table definition.
#[derive(Debug, Serialize)]
pub(crate) struct TableWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Ordered columns.
    columns: Vec<TableColumnWriterV1>,
    /// Required nullable default rows.
    #[serde(serialize_with = "serialize_optional_table_rows")]
    default: Option<Vec<Vec<ShareableString>>>,
}

impl From<&TableDefinition> for TableWriterV1 {
    fn from(table: &TableDefinition) -> Self {
        Self {
            description: table.description(),
            columns: table
                .iter()
                .zip(table.column_requirements())
                .map(
                    |((key, definition), (_, requirement))| TableColumnWriterV1 {
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
