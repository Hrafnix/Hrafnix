use crate::common::serialize::{serialize_shareable_string, serialize_shareable_strings};
use crate::scheme::writer::datastore::definition::common::table::serialize_table_rows;
use crate::scheme::writer::datastore::definition::map::MapDefinitionWriterV1;
use hrafnix_datastore::frozen::{MapFrozen, MapItemFrozen};
use hrafnix_keys::StoreKey;
use hrafnix_shareable_string::ShareableString;
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

/// Untagged map item value serialized without repeating its definition.
#[derive(Debug, Serialize)]
#[serde(untagged)]
enum MapItemValueWriterV1 {
    /// Unevaluated scalar expression.
    Expression(#[serde(serialize_with = "serialize_shareable_string")] ShareableString),
    /// Table reference and row-major cell expressions.
    Table {
        /// Unevaluated table reference, or an empty string for local rows.
        #[serde(serialize_with = "serialize_shareable_string")]
        parameter: ShareableString,
        /// Row-major cell expressions in definition column order.
        #[serde(serialize_with = "serialize_table_rows")]
        rows: Vec<Vec<ShareableString>>,
    },
    /// Numeric expression with its selected units.
    NumberWithUnits {
        /// Current unevaluated numeric expression.
        #[serde(serialize_with = "serialize_shareable_string")]
        value: ShareableString,
        /// Selected unit string.
        #[serde(serialize_with = "serialize_shareable_string")]
        units: ShareableString,
    },
    /// Table cell expressions with units stored once per column.
    TableWithUnits {
        /// Unevaluated table reference, or an empty string for local rows.
        #[serde(serialize_with = "serialize_shareable_string")]
        parameter: ShareableString,
        /// Selected unit strings in definition column order.
        #[serde(serialize_with = "serialize_shareable_strings")]
        units: Vec<ShareableString>,
        /// Row-major cell expressions in definition column order.
        #[serde(serialize_with = "serialize_table_rows")]
        rows: Vec<Vec<ShareableString>>,
    },
}

impl From<&MapItemFrozen> for MapItemValueWriterV1 {
    fn from(item: &MapItemFrozen) -> Self {
        match item {
            MapItemFrozen::Boolean(value) => Self::Expression(value.value()),
            MapItemFrozen::Choice(value) => Self::Expression(value.value()),
            MapItemFrozen::File(value) => Self::Expression(value.value()),
            MapItemFrozen::Integer(value) => Self::Expression(value.value()),
            MapItemFrozen::Number(value) => Self::Expression(value.value()),
            MapItemFrozen::NumberWithUnits(value) => Self::NumberWithUnits {
                value: value.value(),
                units: value.units(),
            },
            MapItemFrozen::String(value) => Self::Expression(value.value()),
            MapItemFrozen::Table(value) => Self::Table {
                parameter: value.parameter().clone(),
                rows: value.rows().to_vec(),
            },
            MapItemFrozen::TableWithUnits(value) => Self::TableWithUnits {
                parameter: value.parameter().clone(),
                units: value.units().to_vec(),
                rows: value.rows().to_vec(),
            },
            MapItemFrozen::Unit(value) => Self::Expression(value.value()),
        }
    }
}

/// Version-1 frozen map with a shared definition and ordered entry values.
#[derive(Debug, Serialize)]
pub(crate) struct MapWriterV1 {
    /// Map metadata, ordered item definitions, and default entries.
    definition: MapDefinitionWriterV1,
    /// Entry keys mapped to item values in definition order.
    #[serde(serialize_with = "serialize_map_values")]
    value: BTreeMap<StoreKey, Vec<MapItemValueWriterV1>>,
}

/// Serializes validated entry keys as JSON object keys.
fn serialize_map_values<S: Serializer>(
    entries: &BTreeMap<StoreKey, Vec<MapItemValueWriterV1>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    entries
        .iter()
        .map(|(key, values)| (key.as_str(), values))
        .collect::<BTreeMap<_, _>>()
        .serialize(serializer)
}

impl From<&MapFrozen> for MapWriterV1 {
    fn from(map: &MapFrozen) -> Self {
        let value = map
            .iter()
            .map(|(entry_key, entry)| {
                let values = map
                    .definition()
                    .keys()
                    .filter_map(|key| entry.get(key.as_str()).map(Into::into))
                    .collect();
                (entry_key.clone(), values)
            })
            .collect();
        Self {
            definition: map.definition().into(),
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::{
        MapDefinition, MapItemDefinition, NumberDefinition, NumberWithUnitsDefinition,
        StringDefinition, TableDefinition, TableWithUnitsDefinition,
    };
    use hrafnix_datastore::frozen::{
        MapEntryFrozen, NumberWithUnitsFrozen, StringFrozen, TableFrozen, TableWithUnitsFrozen,
    };
    use hrafnix_units::UnitId;
    use serde_json::json;

    #[test]
    fn writes_values_without_repeating_item_definitions() {
        let key = |name: &str| StoreKey::new(name.into()).unwrap();
        let label = StringDefinition::new_with_default("", "");
        let distance = NumberWithUnitsDefinition::new_with_default("", "0 m", UnitId::Length_Meter);
        let values = TableWithUnitsDefinition::new("", vec![(key("value"), distance.clone())]);
        let plain = TableDefinition::new(
            "",
            vec![(key("value"), NumberDefinition::new_with_default("", "0"))],
        );
        let definition = MapDefinition::new(
            "Samples",
            vec![
                (key("label"), MapItemDefinition::String(label.clone())),
                (
                    key("distance"),
                    MapItemDefinition::NumberWithUnits(distance.clone()),
                ),
                (
                    key("values"),
                    MapItemDefinition::TableWithUnits(values.clone()),
                ),
                (key("plain"), MapItemDefinition::Table(plain.clone())),
            ],
        );
        let entry = MapEntryFrozen::new_from_items(BTreeMap::from([
            (
                key("label"),
                MapItemFrozen::String(StringFrozen::new_with_value(label, "sample")),
            ),
            (
                key("distance"),
                MapItemFrozen::NumberWithUnits(NumberWithUnitsFrozen::new_with_value(
                    distance,
                    "12 m",
                    "length_centimeter",
                )),
            ),
            (
                key("values"),
                MapItemFrozen::TableWithUnits(TableWithUnitsFrozen::new_from_rows_with_parameter(
                    values,
                    vec![vec!["12 m".into()]],
                    vec!["length_centimeter".into()],
                    "p_units",
                )),
            ),
            (
                key("plain"),
                MapItemFrozen::Table(TableFrozen::new_from_rows_with_parameter(
                    plain,
                    vec![vec!["7".into()], vec!["8 + 9".into()]],
                    "p_plain",
                )),
            ),
        ]));
        let map =
            MapFrozen::new_with_items(definition, BTreeMap::from([(key("first"), entry)])).unwrap();
        let document = json!({
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
                    },
                    {
                        "key": "plain",
                        "type": "table",
                        "description": "",
                        "columns": [{
                            "key": "value", "description": "", "default": "0",
                            "constraint": {"type": "none"}, "requirement": "none"
                        }],
                        "default": null
                    }
                ],
                "default": null
            },
            "value": {
                "first": [
                    "sample",
                    {"value": "12 m", "units": "length_centimeter"},
                    {"parameter": "p_units", "units": ["length_centimeter"], "rows": [["12 m"]]},
                    {"parameter": "p_plain", "rows": [["7"], ["8 + 9"]]}
                ]
            }
        });
        assert_eq!(
            serde_json::to_value(MapWriterV1::from(&map)).unwrap(),
            document
        );
    }
}
