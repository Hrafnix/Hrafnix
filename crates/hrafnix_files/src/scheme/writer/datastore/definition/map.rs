use super::{
    boolean::BooleanDefinitionWriterV1, choice::ChoiceDefinitionWriterV1,
    common::table::serialize_table_rows, file::FileDefinitionWriterV1,
    integer::IntegerDefinitionWriterV1, number::NumberDefinitionWriterV1,
    number_with_units::NumberWithUnitDefinitionWriterV1, string::StringDefinitionWriterV1,
    table::TableDefinitionWriterV1, table_with_units::TableWithUnitsDefinitionWriterV1,
    unit::UnitDefinitionWriterV1,
};
use crate::common::serialize::{serialize_shareable_string, serialize_store_key};
use hrafnix_datastore::definition::{MapDefinition, MapItemDefault, MapItemDefinition};
use hrafnix_keys::StoreKey;
use hrafnix_shareable_string::ShareableString;
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

/// One named item in the ordered version-1 map schema.
#[derive(Debug, Serialize)]
pub(crate) struct MapItemDefinitionWriterV1 {
    /// Plain-string item identifier.
    #[serde(serialize_with = "serialize_store_key")]
    key: StoreKey,
    /// Tagged definition with inline fields.
    #[serde(flatten)]
    definition: MapItemEnumDefinitionWriterV1,
}

/// Version-1 definitions supported inside a map entry.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum MapItemEnumDefinitionWriterV1 {
    /// Boolean expression.
    Boolean(BooleanDefinitionWriterV1),
    /// Choice expression and choices.
    Choice(ChoiceDefinitionWriterV1),
    /// File expression and dialog options.
    File(FileDefinitionWriterV1),
    /// Integer expression and bounds.
    Integer(IntegerDefinitionWriterV1),
    /// Numeric expression and bounds.
    Number(NumberDefinitionWriterV1),
    /// Numeric expression with units.
    NumberWithUnits(NumberWithUnitDefinitionWriterV1),
    /// String expression.
    String(StringDefinitionWriterV1),
    /// Numeric table.
    Table(TableDefinitionWriterV1),
    /// Numeric table with units.
    TableWithUnits(TableWithUnitsDefinitionWriterV1),
    /// Unit selection.
    Unit(UnitDefinitionWriterV1),
}

impl From<&MapItemDefinition> for MapItemEnumDefinitionWriterV1 {
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
enum MapItemDefaultDefinitionWriterV1 {
    /// Scalar expression, including unit expressions.
    Scalar(#[serde(serialize_with = "serialize_shareable_string")] ShareableString),
    /// Table cell expressions.
    Table(#[serde(serialize_with = "serialize_table_rows")] Vec<Vec<ShareableString>>),
}

impl From<&MapItemDefault> for MapItemDefaultDefinitionWriterV1 {
    fn from(default: &MapItemDefault) -> Self {
        match default {
            MapItemDefault::Scalar(value) => Self::Scalar(value.clone()),
            MapItemDefault::Table(rows) => Self::Table(rows.clone()),
        }
    }
}

/// Version-1 map definition with ordered items and explicit nullable defaults.
#[derive(Debug, Serialize)]
pub(crate) struct MapDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Ordered entry schema.
    items: Vec<MapItemDefinitionWriterV1>,
    /// Default entries with plain-string keys.
    #[serde(serialize_with = "serialize_map_defaults")]
    default: Option<BTreeMap<StoreKey, Vec<MapItemDefaultDefinitionWriterV1>>>,
}

/// Serializes default-entry keys as borrowed strings in key order.
#[expect(
    clippy::ref_option,
    reason = "Serde passes a reference to the field type"
)]
fn serialize_map_defaults<S: Serializer>(
    entries: &Option<BTreeMap<StoreKey, Vec<MapItemDefaultDefinitionWriterV1>>>,
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

impl From<&MapDefinition> for MapDefinitionWriterV1 {
    fn from(map: &MapDefinition) -> Self {
        Self {
            description: map.description(),
            items: map
                .iter()
                .map(|(key, definition)| MapItemDefinitionWriterV1 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::{NumberDefinition, StringDefinition, TableDefinition};
    use serde_json::{Value, json};

    fn key(name: &str) -> StoreKey {
        StoreKey::new(name.into()).unwrap()
    }

    fn schema() -> Vec<(StoreKey, MapItemDefinition)> {
        vec![
            (
                key("z"),
                StringDefinition::new_with_default("Label", "base").into(),
            ),
            (
                key("a"),
                TableDefinition::new(
                    "Samples",
                    vec![(
                        key("value"),
                        NumberDefinition::new_with_default("Value", "0"),
                    )],
                )
                .into(),
            ),
        ]
    }

    fn expected(default: &Value) -> Value {
        json!({
            "description": "Entries",
            "items": [
                {"key": "z", "type": "string", "description": "Label", "default": "base"},
                {
                    "key": "a", "type": "table", "description": "Samples",
                    "columns": [{
                        "key": "value", "description": "Value", "default": "0",
                        "constraint": {"type": "none"}, "requirement": "none"
                    }],
                    "default": null
                }
            ],
            "default": default
        })
    }

    #[test]
    fn writes_scalar_and_table_defaults_with_plain_entry_keys() {
        let definition = MapDefinition::new_with_default(
            "Entries",
            schema(),
            BTreeMap::from([
                (
                    key("first"),
                    vec![
                        MapItemDefault::scalar("custom"),
                        MapItemDefault::table(vec![vec!["1"], vec!["2"]]),
                    ],
                ),
                (key("second"), vec![MapItemDefault::scalar("other")]),
            ]),
        );
        assert_eq!(
            serde_json::to_value(MapDefinitionWriterV1::from(&definition)).unwrap(),
            expected(&json!({"first": ["custom", [["1"], ["2"]]], "second": ["other"]}))
        );
    }

    #[test]
    fn preserves_null_and_explicitly_empty_defaults() {
        let absent = MapDefinition::new("Entries", schema());
        let empty = MapDefinition::new_with_default("Entries", schema(), BTreeMap::new());
        assert_eq!(
            serde_json::to_value(MapDefinitionWriterV1::from(&absent)).unwrap(),
            expected(&Value::Null)
        );
        assert_eq!(
            serde_json::to_value(MapDefinitionWriterV1::from(&empty)).unwrap(),
            expected(&json!({}))
        );
    }
}
