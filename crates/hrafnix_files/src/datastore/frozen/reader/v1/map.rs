use crate::common::deserialize::{deserialize_entries, deserialize_object, deserialize_store_key};
use crate::datastore::definition::reader::v1::map::MapDefinitionReaderV1;
use hrafnix_datastore::definition::{MapDefinition, MapItemDefinition};
use hrafnix_datastore::frozen::{
    BooleanFrozen, ChoiceFrozen, FileFrozen, IntegerFrozen, MapEntryFrozen, MapFrozen,
    MapItemFrozen, NumberFrozen, NumberWithUnitsFrozen, StringFrozen, TableFrozen,
    TableWithUnitsFrozen, UnitFrozen,
};
use hrafnix_keys::StoreKey;
use hrafnix_message::message::Message;
use hrafnix_shareable_string::ShareableString;
use serde::de::value::StringDeserializer;
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;

/// Untagged map item value whose type is supplied by the map definition.
#[derive(Debug, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
enum MapItemValueReaderV1 {
    /// Unevaluated scalar expression.
    Expression(String),
    /// Table reference and row-major cell expressions.
    Table {
        /// Unevaluated table reference, or an empty string for local rows.
        parameter: String,
        /// Row-major cell expressions in definition column order.
        rows: Vec<Vec<String>>,
    },
    /// Numeric expression with its selected units.
    NumberWithUnits {
        /// Current unevaluated numeric expression.
        value: String,
        /// Selected unit string.
        units: String,
    },
    /// Table cell expressions with units stored once per column.
    TableWithUnits {
        /// Unevaluated table reference, or an empty string for local rows.
        parameter: String,
        /// Selected unit strings in definition column order.
        units: Vec<String>,
        /// Row-major cell expressions in definition column order.
        rows: Vec<Vec<String>>,
    },
}

impl MapItemValueReaderV1 {
    /// Binds a value to its item definition, rejecting mismatched value shapes.
    fn into_frozen(self, definition: &MapItemDefinition) -> Result<MapItemFrozen, Message> {
        let item = match (definition, self) {
            (MapItemDefinition::Boolean(definition), Self::Expression(value)) => {
                MapItemFrozen::Boolean(BooleanFrozen::new_with_value(definition.clone(), value))
            }
            (MapItemDefinition::Choice(definition), Self::Expression(value)) => {
                MapItemFrozen::Choice(ChoiceFrozen::new_with_value(definition.clone(), value))
            }
            (MapItemDefinition::File(definition), Self::Expression(value)) => {
                MapItemFrozen::File(FileFrozen::new_with_value(definition.clone(), value))
            }
            (MapItemDefinition::Integer(definition), Self::Expression(value)) => {
                MapItemFrozen::Integer(IntegerFrozen::new_with_value(definition.clone(), value))
            }
            (MapItemDefinition::Number(definition), Self::Expression(value)) => {
                MapItemFrozen::Number(NumberFrozen::new_with_value(definition.clone(), value))
            }
            (
                MapItemDefinition::NumberWithUnits(definition),
                Self::NumberWithUnits { value, units },
            ) => MapItemFrozen::NumberWithUnits(NumberWithUnitsFrozen::new_with_value(
                definition.clone(),
                value,
                units,
            )),
            (MapItemDefinition::String(definition), Self::Expression(value)) => {
                MapItemFrozen::String(StringFrozen::new_with_value(definition.clone(), value))
            }
            (MapItemDefinition::Table(definition), Self::Table { parameter, rows }) => {
                MapItemFrozen::Table(TableFrozen::new_from_rows_with_parameter(
                    definition.clone(),
                    into_rows(rows),
                    parameter,
                ))
            }
            (
                MapItemDefinition::TableWithUnits(definition),
                Self::TableWithUnits {
                    parameter,
                    units,
                    rows,
                },
            ) => {
                if units.len() != definition.count() {
                    return Err(map_value_error());
                }
                MapItemFrozen::TableWithUnits(TableWithUnitsFrozen::new_from_rows_with_parameter(
                    definition.clone(),
                    into_rows(rows),
                    units.into_iter().map(ShareableString::from).collect(),
                    parameter,
                ))
            }
            (MapItemDefinition::Unit(definition), Self::Expression(value)) => {
                MapItemFrozen::Unit(UnitFrozen::new_with_value(definition.clone(), value))
            }
            _ => return Err(map_value_error()),
        };
        Ok(item)
    }
}

/// Converts owned table cell strings into shareable strings.
fn into_rows(rows: Vec<Vec<String>>) -> Vec<Vec<ShareableString>> {
    rows.into_iter()
        .map(|row| row.into_iter().map(ShareableString::from).collect())
        .collect()
}

/// Map entry values with validated keys and duplicate-key rejection.
#[derive(Debug)]
struct MapValueReaderV1 {
    /// Entry keys mapped to item values in definition order.
    entries: BTreeMap<StoreKey, Vec<MapItemValueReaderV1>>,
}

impl<'de> Deserialize<'de> for MapValueReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let entries = deserialize_entries::<Vec<MapItemValueReaderV1>, D>(deserializer)?
            .into_iter()
            .map(|(key, values)| {
                let key = deserialize_store_key(StringDeserializer::<D::Error>::new(key))?;
                Ok((key, values))
            })
            .collect::<Result<_, D::Error>>()?;
        Ok(Self { entries })
    }
}

/// Version-1 frozen map with a shared definition and ordered entry values.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MapReaderV1 {
    /// Map metadata, ordered item definitions, and default entries.
    #[serde(deserialize_with = "deserialize_object")]
    definition: MapDefinitionReaderV1,
    /// Entry values to bind to the shared item definitions.
    value: MapValueReaderV1,
}

impl TryFrom<MapReaderV1> for MapFrozen {
    type Error = Message;

    fn try_from(map: MapReaderV1) -> Result<Self, Self::Error> {
        let definition: MapDefinition = map.definition.into();
        let items = map
            .value
            .entries
            .into_iter()
            .map(|(entry_key, values)| {
                if values.len() != definition.count() {
                    return Err(map_value_error());
                }
                let items = definition
                    .iter()
                    .zip(values)
                    .map(|((key, item_definition), value)| {
                        value
                            .into_frozen(item_definition)
                            .map(|item| (key.clone(), item))
                    })
                    .collect::<Result<_, _>>()?;
                Ok((entry_key, MapEntryFrozen::new_from_items(items)))
            })
            .collect::<Result<_, _>>()?;
        Self::new_with_items(definition, items)
    }
}

/// Reports an entry value shape or item count that does not match the definition.
fn map_value_error() -> Message {
    Message::error(
        hrafnix_message::message::MessageCategory::Datastore,
        "datastore_schema_mismatch",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use serde_json::json;

    #[test]
    fn reuses_map_definition_for_entry_values() {
        let reader = serde_json::from_value::<RootObject<MapReaderV1>>(json!({
            "definition": {
                "description": "Samples",
                "items": [
                    {"key": "label", "type": "string", "description": "", "default": ""},
                    {
                        "key": "distance",
                        "type": "number_with_units",
                        "description": "",
                        "default": "0 m",
                        "constraint": {"type": "none"},
                        "preferred_unit": "length_meter"
                    },
                    {
                        "key": "values",
                        "type": "table_with_units",
                        "description": "",
                        "columns": [{
                            "key": "value",
                            "description": "",
                            "default": "0 m",
                            "constraint": {"type": "none"},
                            "requirement": "none",
                            "preferred_unit": "length_meter"
                        }],
                        "default": null
                    }
                ],
                "default": null
            },
            "value": {
                "first": [
                    "sample",
                    {"value": "12 m", "units": "length_meter"},
                    {"parameter": "", "units": ["length_meter"], "rows": [["12 m"]]}
                ]
            }
        }))
        .expect("valid map value")
        .document;

        let map = MapFrozen::try_from(reader).expect("matching schema");
        let entry = map.get("first").expect("entry");
        assert_eq!(
            entry.get_string("label").expect("string value").value(),
            "sample"
        );
        let Some(MapItemFrozen::NumberWithUnits(distance)) = entry.get("distance") else {
            panic!("number with units");
        };
        assert_eq!(distance.value(), "12 m");
        assert_eq!(distance.units(), "length_meter");
        let Some(MapItemFrozen::TableWithUnits(values)) = entry.get("values") else {
            panic!("table with units");
        };
        assert_eq!(values.rows()[0][0], "12 m");
        assert_eq!(values.units()[0], "length_meter");
    }

    #[test]
    fn rejects_values_that_do_not_match_the_definition() {
        let reader = serde_json::from_value::<RootObject<MapReaderV1>>(json!({
            "definition": {
                "description": "",
                "items": [
                    {"key": "name", "type": "string", "description": "", "default": ""}
                ],
                "default": null
            },
            "value": {"first": []}
        }))
        .expect("structurally valid map value")
        .document;

        assert!(MapFrozen::try_from(reader).is_err());
    }
}
