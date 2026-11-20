//! Version-1 maps use ordered `items` with a unique `key`, a snake-case `type`, and
//! inline definition fields. The required `default` is null or an object whose
//! validated entry keys map to ordered scalar strings or table row arrays. Entries
//! may omit trailing items. The datastore definition normalizes extra defaults,
//! scalar/table mismatches, and table row widths.

use crate::common::deserialize::{StringTagDeserializer, deserialize_entries, deserialize_objects};
use crate::common::validate::{ensure_unique_keys, unique_entries};
use crate::datastore::definition::reader::v1::{
    boolean::BooleanDefinitionReaderV1, choice::ChoiceDefinitionReaderV1,
    file::FileDefinitionReaderV1, integer::IntegerDefinitionReaderV1,
    number::NumberDefinitionReaderV1, number_with_units::NumberWithUnitDefinitionReaderV1,
    string::StringDefinitionReaderV1, table::TableDefinitionReaderV1,
    table_with_units::TableWithUnitsDefinitionReaderV1, unit::UnitDefinitionReaderV1,
};
use hrafnix_datastore::definition::{MapDefinition, MapItemDefault, MapItemDefinition};
use hrafnix_keys::StoreKey;
use hrafnix_message::message::Message;
use serde::{Deserialize, Deserializer};

/// One named item in the ordered map entry schema.
#[derive(Debug, Deserialize)]
pub(crate) struct MapItemDefinitionReaderV1 {
    /// Plain-string item identifier, validated during conversion.
    key: String,
    /// Tagged definition; each variant rejects unknown fields.
    #[serde(flatten)]
    definition: MapItemEnumDefinitionReaderV1,
}

/// Definitions supported inside a version-1 map entry.
#[derive(Debug, Deserialize)]
#[serde(remote = "Self", tag = "type", rename_all = "snake_case")]
enum MapItemEnumDefinitionReaderV1 {
    /// Boolean expression.
    Boolean(BooleanDefinitionReaderV1),
    /// Choice expression and choices.
    Choice(ChoiceDefinitionReaderV1),
    /// File expression and dialog options.
    File(FileDefinitionReaderV1),
    /// Integer expression and bounds.
    Integer(IntegerDefinitionReaderV1),
    /// Numeric expression and bounds.
    Number(NumberDefinitionReaderV1),
    /// Numeric expression with units.
    NumberWithUnits(NumberWithUnitDefinitionReaderV1),
    /// String expression.
    String(StringDefinitionReaderV1),
    /// Numeric table.
    Table(TableDefinitionReaderV1),
    /// Numeric table with units.
    TableWithUnits(TableWithUnitsDefinitionReaderV1),
    /// Unit selection.
    Unit(UnitDefinitionReaderV1),
}

impl<'de> Deserialize<'de> for MapItemEnumDefinitionReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::deserialize(StringTagDeserializer(deserializer))
    }
}

impl TryFrom<MapItemEnumDefinitionReaderV1> for MapItemDefinition {
    type Error = Message;

    fn try_from(definition: MapItemEnumDefinitionReaderV1) -> Result<Self, Self::Error> {
        Ok(match definition {
            MapItemEnumDefinitionReaderV1::Boolean(item) => Self::Boolean(item.into()),
            MapItemEnumDefinitionReaderV1::Choice(item) => Self::Choice(item.try_into()?),
            MapItemEnumDefinitionReaderV1::File(item) => Self::File(item.into()),
            MapItemEnumDefinitionReaderV1::Integer(item) => Self::Integer(item.into()),
            MapItemEnumDefinitionReaderV1::Number(item) => Self::Number(item.into()),
            MapItemEnumDefinitionReaderV1::NumberWithUnits(item) => {
                Self::NumberWithUnits(item.into())
            }
            MapItemEnumDefinitionReaderV1::String(item) => Self::String(item.into()),
            MapItemEnumDefinitionReaderV1::Table(item) => Self::Table(item.try_into()?),
            MapItemEnumDefinitionReaderV1::TableWithUnits(item) => {
                Self::TableWithUnits(item.try_into()?)
            }
            MapItemEnumDefinitionReaderV1::Unit(item) => Self::Unit(item.into()),
        })
    }
}

/// Scalar expression or table rows for one item in a default entry.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MapItemDefaultDefinitionReaderV1 {
    /// Scalar expression, including unit expressions.
    Scalar(String),
    /// Table cell expressions.
    Table(Vec<Vec<String>>),
}

impl From<MapItemDefaultDefinitionReaderV1> for MapItemDefault {
    fn from(default: MapItemDefaultDefinitionReaderV1) -> Self {
        match default {
            MapItemDefaultDefinitionReaderV1::Scalar(value) => Self::scalar(value),
            MapItemDefaultDefinitionReaderV1::Table(rows) => Self::table(rows),
        }
    }
}

/// Default entries in file order; keys are validated during conversion.
#[derive(Debug)]
struct MapDefaultsDefinitionReaderV1 {
    /// Ordered item defaults for each named entry.
    entries: Vec<(String, Vec<MapItemDefaultDefinitionReaderV1>)>,
}

impl<'de> Deserialize<'de> for MapDefaultsDefinitionReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let entries = deserialize_entries(deserializer)?;
        Ok(Self { entries })
    }
}

/// Version-1 map definition.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MapDefinitionReaderV1 {
    /// Item description.
    description: String,
    /// Ordered entry schema; item keys are checked for uniqueness during conversion.
    #[serde(deserialize_with = "deserialize_objects")]
    items: Vec<MapItemDefinitionReaderV1>,
    /// Required nullable default entries. Each entry may omit trailing items, which then
    /// inherit their item definition's default.
    #[serde(deserialize_with = "Option::deserialize")]
    default: Option<MapDefaultsDefinitionReaderV1>,
}

impl TryFrom<MapDefinitionReaderV1> for MapDefinition {
    type Error = Message;

    fn try_from(map: MapDefinitionReaderV1) -> Result<Self, Self::Error> {
        ensure_unique_keys(
            map.items.iter().map(|item| &item.key),
            "files_duplicate_map_item_key",
        )?;
        let items: Vec<_> = map
            .items
            .into_iter()
            .map(|item| {
                Ok((
                    StoreKey::new(item.key.into())?,
                    MapItemDefinition::try_from(item.definition)?,
                ))
            })
            .collect::<Result<_, Message>>()?;
        Ok(match map.default {
            Some(default) => {
                let entries = default
                    .entries
                    .into_iter()
                    .map(|(key, values)| {
                        Ok((
                            StoreKey::new(key.into())?,
                            values.into_iter().map(Into::into).collect(),
                        ))
                    })
                    .collect::<Result<Vec<_>, Message>>()?;
                let entries = unique_entries(entries, "files_duplicate_map_entry_key")?;
                Self::new_with_default(map.description, items, entries)
            }
            None => Self::new(map.description, items),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_support::{ReadError, read_owned_value, read_str};
    use hrafnix_datastore::definition::{
        BooleanDefinition, ChoiceDefinition, ChoiceItemDefinition, FileDefinition,
        IntegerConstraint, IntegerDefinition, NumberConstraint, NumberDefinition,
        NumberWithUnitsDefinition, StringDefinition, TableDefinition, TableWithUnitsDefinition,
        UnitDefinition,
    };
    use hrafnix_units::{UnitFamilyId, UnitId};
    use serde_json::{Value, json};
    use std::collections::BTreeMap;

    fn key(value: &str) -> StoreKey {
        StoreKey::new(value.into()).expect("valid store key")
    }

    fn read_map(value: Value) -> MapDefinition {
        try_read(value).expect("valid map definition")
    }

    fn try_read(value: Value) -> Result<MapDefinition, ReadError> {
        read_owned_value::<MapDefinitionReaderV1, _>(value)
    }

    fn try_read_str(text: &str) -> Result<MapDefinition, ReadError> {
        read_str::<MapDefinitionReaderV1, _>(text)
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
            assert!(try_read(value).is_err(), "{field}");
        }
        let mut value = map_json(&Value::Null);
        value["unexpected"] = json!(true);
        assert!(try_read(value).is_err());

        for field in ["key", "type", "description", "default"] {
            let mut value = map_json(&Value::Null);
            value["items"][0].as_object_mut().unwrap().remove(field);
            assert!(try_read(value).is_err(), "{field}");
        }
        for index in 0..3 {
            let mut value = map_json(&Value::Null);
            value["items"][index]["unexpected"] = json!(true);
            assert!(try_read(value).is_err());
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
            assert!(try_read(value).is_err());
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
            assert!(try_read(map_json(&default)).is_err());
        }
    }

    #[test]
    fn rejects_duplicate_item_keys() {
        let mut value = map_json(&Value::Null);
        value["items"][2]["key"] = json!("z");
        let error = try_read(value)
            .expect_err("duplicate item key must be rejected")
            .message();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_duplicate_map_item_key"
        );
        assert_eq!(error.translate_data().message_params()["key"].as_str(), "z");
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
            assert!(try_read_str(text).is_err(), "{text}");
        }
    }
}
