use super::{
    boolean::BooleanWriterV1, choice::ChoiceWriterV1, common::table::serialize_table_rows,
    file::FileWriterV1, integer::IntegerWriterV1, number::NumberWriterV1,
    number_with_units::NumberWithUnitWriterV1, string::StringWriterV1, table::TableWriterV1,
    table_with_units::TableWithUnitsWriterV1, unit::UnitWriterV1,
};
use crate::common::serialize::{serialize_shareable_string, serialize_store_key};
use hrafnix_datastore::definition::{MapDefinition, MapItemDefault, MapItemDefinition};
use hrafnix_keys::StoreKey;
use hrafnix_shareable_string::ShareableString;
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

/// One named item in the ordered version-1 map schema.
#[derive(Debug, Serialize)]
pub(crate) struct MapItemWriterV1 {
    /// Plain-string item identifier.
    #[serde(serialize_with = "serialize_store_key")]
    key: StoreKey,
    /// Tagged definition with inline fields.
    #[serde(flatten)]
    definition: MapItemDefinitionWriterV1,
}

/// Version-1 definitions supported inside a map entry.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum MapItemDefinitionWriterV1 {
    /// Boolean expression.
    Boolean(BooleanWriterV1),
    /// Choice expression and choices.
    Choice(ChoiceWriterV1),
    /// File expression and dialog options.
    File(FileWriterV1),
    /// Integer expression and bounds.
    Integer(IntegerWriterV1),
    /// Numeric expression and bounds.
    Number(NumberWriterV1),
    /// Numeric expression with units.
    NumberWithUnits(NumberWithUnitWriterV1),
    /// String expression.
    String(StringWriterV1),
    /// Numeric table.
    Table(TableWriterV1),
    /// Numeric table with units.
    TableWithUnits(TableWithUnitsWriterV1),
    /// Unit selection.
    Unit(UnitWriterV1),
}

impl From<&MapItemDefinition> for MapItemDefinitionWriterV1 {
    fn from(definition: &MapItemDefinition) -> Self {
        match definition {
            MapItemDefinition::Boolean(item) => Self::Boolean(item.into()),
            MapItemDefinition::Choice(item) => Self::Choice(item.into()),
            MapItemDefinition::File(item) => Self::File(item.into()),
            MapItemDefinition::Integer(item) => Self::Integer(item.into()),
            MapItemDefinition::Number(item) => Self::Number(item.into()),
            MapItemDefinition::NumberWithUnits(item) => Self::NumberWithUnits(item.into()),
            MapItemDefinition::String(item) => Self::String(item.into()),
            MapItemDefinition::Table(item) => Self::Table(item.into()),
            MapItemDefinition::TableWithUnits(item) => Self::TableWithUnits(item.into()),
            MapItemDefinition::Unit(item) => Self::Unit(item.into()),
        }
    }
}

/// Scalar expression or table rows for one item in a default entry.
#[derive(Debug, Serialize)]
#[serde(untagged)]
enum MapItemDefaultWriterV1 {
    /// Scalar expression, including unit expressions.
    Scalar(#[serde(serialize_with = "serialize_shareable_string")] ShareableString),
    /// Table cell expressions.
    Table(#[serde(serialize_with = "serialize_table_rows")] Vec<Vec<ShareableString>>),
}

impl From<&MapItemDefault> for MapItemDefaultWriterV1 {
    fn from(default: &MapItemDefault) -> Self {
        match default {
            MapItemDefault::Scalar(value) => Self::Scalar(value.clone()),
            MapItemDefault::Table(rows) => Self::Table(rows.clone()),
        }
    }
}

/// Version-1 map definition with ordered items and explicit nullable defaults.
#[derive(Debug, Serialize)]
pub(crate) struct MapWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Ordered entry schema.
    items: Vec<MapItemWriterV1>,
    /// Default entries with plain-string keys.
    #[serde(serialize_with = "serialize_map_defaults")]
    default: Option<BTreeMap<StoreKey, Vec<MapItemDefaultWriterV1>>>,
}

/// Serializes default-entry keys as borrowed strings in key order.
#[expect(
    clippy::ref_option,
    reason = "Serde passes a reference to the field type"
)]
fn serialize_map_defaults<S: Serializer>(
    entries: &Option<BTreeMap<StoreKey, Vec<MapItemDefaultWriterV1>>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    entries
        .as_ref()
        .map(|entries| {
            entries
                .iter()
                .map(|(key, values)| (key.as_str(), values))
                .collect::<BTreeMap<_, _>>()
        })
        .serialize(serializer)
}

impl From<&MapDefinition> for MapWriterV1 {
    fn from(map: &MapDefinition) -> Self {
        Self {
            description: map.description(),
            items: map
                .iter()
                .map(|(key, definition)| MapItemWriterV1 {
                    key: key.clone(),
                    definition: definition.into(),
                })
                .collect(),
            default: map.default_map().map(|entries| {
                entries
                    .iter()
                    .map(|(key, values)| (key.clone(), values.iter().map(Into::into).collect()))
                    .collect()
            }),
        }
    }
}
