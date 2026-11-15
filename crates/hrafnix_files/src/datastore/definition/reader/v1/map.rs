//! Version-1 maps use ordered `items` with a unique `key`, a snake-case `type`, and
//! inline definition fields. The required `default` is null or an object whose
//! validated entry keys map to ordered scalar strings or table row arrays. Entries
//! may omit trailing items. The datastore definition normalizes extra defaults,
//! scalar/table mismatches, and table row widths.

use crate::common::deserialize::{
    StringTagDeserializer, UniqueKey, deserialize_entries, deserialize_store_key,
    deserialize_unique_objects,
};
use crate::datastore::definition::reader::v1::{
    boolean::BooleanReaderV1, choice::ChoiceReaderV1, file::FileReaderV1, integer::IntegerReaderV1,
    number::NumberReaderV1, number_with_units::NumberWithUnitReaderV1, string::StringReaderV1,
    table::TableReaderV1, table_with_units::TableWithUnitsReaderV1, unit::UnitReaderV1,
};
use hrafnix_datastore::definition::{MapDefinition, MapItemDefault, MapItemDefinition};
use hrafnix_keys::StoreKey;
use serde::de::value::StringDeserializer;
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;

/// One named item in the ordered map entry schema.
#[derive(Debug, Deserialize)]
pub(crate) struct MapItemReaderV1 {
    /// Validated item identifier.
    #[serde(deserialize_with = "deserialize_store_key")]
    key: StoreKey,
    /// Tagged definition; each variant rejects unknown fields.
    #[serde(flatten)]
    definition: MapItemDefinitionReaderV1,
}

impl UniqueKey for MapItemReaderV1 {
    const KEY_NAME: &'static str = "map item key";

    fn unique_key(&self) -> &StoreKey {
        &self.key
    }
}

/// Definitions supported inside a version-1 map entry.
#[derive(Debug, Deserialize)]
#[serde(remote = "Self", tag = "type", rename_all = "snake_case")]
enum MapItemDefinitionReaderV1 {
    /// Boolean expression.
    Boolean(BooleanReaderV1),
    /// Choice expression and choices.
    Choice(ChoiceReaderV1),
    /// File expression and dialog options.
    File(FileReaderV1),
    /// Integer expression and bounds.
    Integer(IntegerReaderV1),
    /// Numeric expression and bounds.
    Number(NumberReaderV1),
    /// Numeric expression with units.
    NumberWithUnits(NumberWithUnitReaderV1),
    /// String expression.
    String(StringReaderV1),
    /// Numeric table.
    Table(TableReaderV1),
    /// Numeric table with units.
    TableWithUnits(TableWithUnitsReaderV1),
    /// Unit selection.
    Unit(UnitReaderV1),
}

impl<'de> Deserialize<'de> for MapItemDefinitionReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::deserialize(StringTagDeserializer(deserializer))
    }
}

impl From<MapItemDefinitionReaderV1> for MapItemDefinition {
    fn from(definition: MapItemDefinitionReaderV1) -> Self {
        match definition {
            MapItemDefinitionReaderV1::Boolean(item) => Self::Boolean(item.into()),
            MapItemDefinitionReaderV1::Choice(item) => Self::Choice(item.into()),
            MapItemDefinitionReaderV1::File(item) => Self::File(item.into()),
            MapItemDefinitionReaderV1::Integer(item) => Self::Integer(item.into()),
            MapItemDefinitionReaderV1::Number(item) => Self::Number(item.into()),
            MapItemDefinitionReaderV1::NumberWithUnits(item) => Self::NumberWithUnits(item.into()),
            MapItemDefinitionReaderV1::String(item) => Self::String(item.into()),
            MapItemDefinitionReaderV1::Table(item) => Self::Table(item.into()),
            MapItemDefinitionReaderV1::TableWithUnits(item) => Self::TableWithUnits(item.into()),
            MapItemDefinitionReaderV1::Unit(item) => Self::Unit(item.into()),
        }
    }
}

/// Scalar expression or table rows for one item in a default entry.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MapItemDefaultReaderV1 {
    /// Scalar expression, including unit expressions.
    Scalar(String),
    /// Table cell expressions.
    Table(Vec<Vec<String>>),
}

impl From<MapItemDefaultReaderV1> for MapItemDefault {
    fn from(default: MapItemDefaultReaderV1) -> Self {
        match default {
            MapItemDefaultReaderV1::Scalar(value) => Self::scalar(value),
            MapItemDefaultReaderV1::Table(rows) => Self::table(rows),
        }
    }
}

/// Default entries with validated, unique store keys.
#[derive(Debug)]
struct MapDefaultsReaderV1 {
    /// Ordered item defaults for each named entry.
    entries: BTreeMap<StoreKey, Vec<MapItemDefaultReaderV1>>,
}

impl<'de> Deserialize<'de> for MapDefaultsReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let entries = deserialize_entries::<Vec<MapItemDefaultReaderV1>, D>(deserializer)?
            .into_iter()
            .map(|(key, values)| {
                let key = deserialize_store_key(StringDeserializer::<D::Error>::new(key))?;
                Ok((key, values))
            })
            .collect::<Result<_, D::Error>>()?;
        Ok(Self { entries })
    }
}

/// Version-1 map definition.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MapReaderV1 {
    /// Item description.
    description: String,
    /// Ordered entry schema with unique item keys.
    #[serde(deserialize_with = "deserialize_unique_objects")]
    items: Vec<MapItemReaderV1>,
    /// Required nullable default entries. Each entry may omit trailing items, which then
    /// inherit their item definition's default.
    #[serde(deserialize_with = "Option::deserialize")]
    default: Option<MapDefaultsReaderV1>,
}

impl From<MapReaderV1> for MapDefinition {
    fn from(map: MapReaderV1) -> Self {
        let items: Vec<_> = map
            .items
            .into_iter()
            .map(|item| (item.key, MapItemDefinition::from(item.definition)))
            .collect();
        match map.default {
            Some(default) => {
                let entries = default
                    .entries
                    .into_iter()
                    .map(|(key, values)| (key, values.into_iter().map(Into::into).collect()))
                    .collect();
                Self::new_with_default(map.description, items, entries)
            }
            None => Self::new(map.description, items),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::{
        BooleanDefinition, ChoiceDefinition, ChoiceItemDefinition, FileDefinition,
        IntegerConstraint, IntegerDefinition, NumberConstraint, NumberDefinition,
        NumberWithUnitsDefinition, StringDefinition, TableDefinition, TableWithUnitsDefinition,
        UnitDefinition,
    };
    use hrafnix_units::{UnitFamilyId, UnitId};
    use serde_json::{Value, json};

    fn key(value: &str) -> StoreKey {
        StoreKey::new(value.into()).expect("valid store key")
    }

    fn read_map(value: Value) -> MapDefinition {
        serde_json::from_value::<RootObject<MapReaderV1>>(value)
            .expect("valid map definition")
            .document
            .into()
    }

    fn map_json(default: &Value) -> Value {
        json!({
            "description": "Map",
            "items": [
                {"key": "z", "type": "string", "description": "Label", "default": "base"},
                {
                    "key": "a",
                    "type": "table",
                    "description": "Samples",
                    "columns": [{
                        "key": "value", "description": "", "default": "0",
                        "constraint": {"type": "none"}, "requirement": "increasing"
                    }],
                    "default": null
                },
                {
                    "key": "b",
                    "type": "table_with_units",
                    "description": "Positions",
                    "columns": [{
                        "key": "distance", "description": "", "default": "0 m",
                        "constraint": {"type": "none"}, "requirement": "none",
                        "preferred_unit": "length_meter"
                    }],
                    "default": null
                }
            ],
            "default": default
        })
    }

    #[test]
    fn converts_every_supported_item_type() {
        let cases: Vec<(Value, MapItemDefinition)> = vec![
            (
                json!({
                    "type": "boolean", "description": "Flag", "default": "1 == 1",
                    "true_description": "On", "false_description": "Off"
                }),
                BooleanDefinition::new_with_descriptions_and_default("Flag", "On", "Off", "1 == 1")
                    .into(),
            ),
            (
                json!({
                    "type": "choice", "description": "Mode", "default": "fast",
                    "items": [{"id": "fast", "description": "Fast"}]
                }),
                ChoiceDefinition::new_with_default(
                    "Mode",
                    vec![ChoiceItemDefinition::new(key("fast"), "Fast")],
                    "fast",
                )
                .into(),
            ),
            (
                json!({
                    "type": "file", "description": "Input", "default": "input.csv",
                    "extension_filter": "*.csv", "is_input": true
                }),
                FileDefinition::new_with_default("Input", "*.csv", true, "input.csv").into(),
            ),
            (
                json!({
                    "type": "integer", "description": "Count", "default": "2 + 3",
                    "constraint": {"type": "min", "min": 0, "inclusive": true}
                }),
                IntegerDefinition::new_with_constraint_and_default(
                    "Count",
                    IntegerConstraint::min(0, true),
                    "2 + 3",
                )
                .into(),
            ),
            (
                json!({
                    "type": "number", "description": "Value", "default": "2.5",
                    "constraint": {"type": "max", "max": 10.0, "inclusive": false}
                }),
                NumberDefinition::new_with_constraint_and_default(
                    "Value",
                    NumberConstraint::max(10.0, false),
                    "2.5",
                )
                .into(),
            ),
            (
                json!({
                    "type": "number_with_units", "description": "Distance", "default": "2 m",
                    "constraint": {"type": "none"}, "preferred_unit": "length_meter"
                }),
                MapItemDefinition::NumberWithUnits(
                    NumberWithUnitsDefinition::new_with_constraint_and_default(
                        "Distance",
                        NumberConstraint::none(),
                        "2 m",
                        UnitId::Length_Meter,
                    ),
                ),
            ),
            (
                json!({"type": "string", "description": "Label", "default": "text"}),
                StringDefinition::new_with_default("Label", "text").into(),
            ),
            (
                json!({"type": "table", "description": "Samples", "columns": [], "default": null}),
                TableDefinition::new("Samples", Vec::<(StoreKey, NumberDefinition)>::new()).into(),
            ),
            (
                json!({
                    "type": "table_with_units", "description": "Positions",
                    "columns": [], "default": null
                }),
                TableWithUnitsDefinition::new(
                    "Positions",
                    Vec::<(StoreKey, NumberWithUnitsDefinition)>::new(),
                )
                .into(),
            ),
            (
                json!({
                    "type": "unit", "description": "Units", "default": "u_length_meter",
                    "unit_family": "length"
                }),
                UnitDefinition::new_with_default("Units", UnitFamilyId::Length, "u_length_meter")
                    .into(),
            ),
        ];
        let mut items = Vec::new();
        let mut expected = Vec::new();
        for (index, (mut value, definition)) in cases.into_iter().enumerate() {
            let name = format!("item{}", 10 - index);
            value["key"] = json!(name);
            items.push(value);
            expected.push((key(&name), definition));
        }
        let map = read_map(json!({"description": "All types", "items": items, "default": null}));
        assert_eq!(map, MapDefinition::new("All types", expected));
    }

    #[test]
    fn preserves_order_and_mixed_entry_defaults() {
        let map = read_map(map_json(&json!({
            "first": ["custom", [["1"], ["2"]], [["3 m"]]],
            "second": ["other", [], []]
        })));
        assert_eq!(
            map.keys().map(StoreKey::as_str).collect::<Vec<_>>(),
            ["z", "a", "b"]
        );
        assert_eq!(
            map.default_map(),
            Some(&BTreeMap::from([
                (
                    key("first"),
                    vec![
                        MapItemDefault::scalar("custom"),
                        MapItemDefault::table(vec![vec!["1"], vec!["2"]]),
                        MapItemDefault::table(vec![vec!["3 m"]]),
                    ],
                ),
                (
                    key("second"),
                    vec![
                        MapItemDefault::scalar("other"),
                        MapItemDefault::table(Vec::<Vec<String>>::new()),
                        MapItemDefault::table(Vec::<Vec<String>>::new()),
                    ],
                ),
            ]))
        );
    }

    #[test]
    fn distinguishes_null_and_empty_defaults() {
        assert_eq!(read_map(map_json(&Value::Null)).default_map(), None);
        assert_eq!(
            read_map(map_json(&json!({}))).default_map(),
            Some(&BTreeMap::new())
        );
        assert_eq!(
            read_map(json!({"description": "", "items": [], "default": {}})).count(),
            0
        );
    }

    #[test]
    fn requires_explicit_fields_and_rejects_unknown_fields() {
        for field in ["description", "items", "default"] {
            let mut value = map_json(&Value::Null);
            value.as_object_mut().unwrap().remove(field);
            assert!(
                serde_json::from_value::<RootObject<MapReaderV1>>(value).is_err(),
                "{field}"
            );
        }
        let mut value = map_json(&Value::Null);
        value["unexpected"] = json!(true);
        assert!(serde_json::from_value::<RootObject<MapReaderV1>>(value).is_err());

        for field in ["key", "type", "description", "default"] {
            let mut value = map_json(&Value::Null);
            value["items"][0].as_object_mut().unwrap().remove(field);
            assert!(
                serde_json::from_value::<RootObject<MapReaderV1>>(value).is_err(),
                "{field}"
            );
        }
        for index in 0..3 {
            let mut value = map_json(&Value::Null);
            value["items"][index]["unexpected"] = json!(true);
            assert!(serde_json::from_value::<RootObject<MapReaderV1>>(value).is_err());
        }
    }

    #[test]
    fn rejects_invalid_schema_items() {
        for item in [
            json!(["z", "string", "Label", "base"]),
            json!({"key": "invalid key", "type": "string", "description": "", "default": ""}),
            json!({"key": "z", "type": "map", "description": "", "items": [], "default": null}),
            json!({"key": "z", "type": "folder", "description": "", "items": []}),
            json!({"key": "z", "type": {"string": null}, "description": "", "default": ""}),
            json!({"key": "z", "type": "string", "description": "", "default": 1}),
            json!({
                "key": "z", "type": "unit", "description": "", "default": "",
                "unit_family": {"length": null}
            }),
        ] {
            let mut value = map_json(&Value::Null);
            value["items"][0] = item;
            assert!(serde_json::from_value::<RootObject<MapReaderV1>>(value).is_err());
        }
    }

    #[test]
    fn rejects_invalid_entry_defaults() {
        for default in [
            json!([]),
            json!({"invalid key": []}),
            json!({"first": null}),
            json!({"first": {}}),
            json!({"first": [true]}),
            json!({"first": [1]}),
            json!({"first": [null]}),
            json!({"first": [[1]]}),
            json!({"first": [[["1", false]]]}),
        ] {
            assert!(serde_json::from_value::<RootObject<MapReaderV1>>(map_json(&default)).is_err());
        }
    }

    #[test]
    fn rejects_duplicate_item_keys() {
        let mut value = map_json(&Value::Null);
        value["items"][2]["key"] = json!("z");
        let error = serde_json::from_value::<RootObject<MapReaderV1>>(value)
            .expect_err("duplicate item key must be rejected");
        assert!(
            error.to_string().contains("duplicate map item key `z`"),
            "{error}"
        );
    }

    #[test]
    fn accepts_entries_that_omit_trailing_items() {
        let map = read_map(map_json(&json!({"first": ["custom"], "second": []})));
        assert_eq!(
            map.default_map(),
            Some(&BTreeMap::from([
                (key("first"), vec![MapItemDefault::scalar("custom")]),
                (key("second"), Vec::new()),
            ]))
        );
    }

    #[test]
    fn normalizes_entry_defaults_through_the_definition() {
        for (default, expected) in [
            (
                json!({"first": ["custom", [], [], "extra"]}),
                vec![
                    MapItemDefault::scalar("custom"),
                    MapItemDefault::table(Vec::<Vec<String>>::new()),
                    MapItemDefault::table(Vec::<Vec<String>>::new()),
                ],
            ),
            (
                json!({"first": [[["1"]]]}),
                vec![MapItemDefault::scalar("base")],
            ),
            (
                json!({"first": ["custom", "1"]}),
                vec![
                    MapItemDefault::scalar("custom"),
                    MapItemDefault::table(Vec::<Vec<String>>::new()),
                ],
            ),
            (
                json!({"first": ["custom", [["1", "2"]]]}),
                vec![
                    MapItemDefault::scalar("custom"),
                    MapItemDefault::table(vec![vec!["1"]]),
                ],
            ),
            (
                json!({"first": ["custom", [], [["1 m"], []]]}),
                vec![
                    MapItemDefault::scalar("custom"),
                    MapItemDefault::table(Vec::<Vec<String>>::new()),
                    MapItemDefault::table(vec![vec!["1 m"], vec![""]]),
                ],
            ),
        ] {
            let map = read_map(map_json(&default));
            assert_eq!(
                map.default_map(),
                Some(&BTreeMap::from([(key("first"), expected)])),
                "{default}"
            );
        }
    }

    #[test]
    fn rejects_duplicate_json_fields_and_entry_keys() {
        for text in [
            r#"{"description":"","items":[],"default":{"entry":[],"entry":[]}}"#,
            r#"{"description":"","items":[],"default":null,"default":{}}"#,
            r#"{"description":"","items":[{"key":"a","key":"b","type":"string","description":"","default":""}],"default":null}"#,
            r#"{"description":"","items":[{"key":"a","type":"string","type":"string","description":"","default":""}],"default":null}"#,
            r#"{"description":"","items":[{"key":"a","type":"string","description":"","default":"","default":""}],"default":null}"#,
        ] {
            assert!(
                serde_json::from_str::<RootObject<MapReaderV1>>(text).is_err(),
                "{text}"
            );
        }
    }
}
