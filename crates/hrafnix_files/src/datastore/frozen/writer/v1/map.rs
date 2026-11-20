use crate::common::serialize::{serialize_shareable_string, serialize_shareable_strings};
use crate::datastore::definition::writer::common::table::serialize_table_rows;
use crate::datastore::definition::writer::map::MapDefinitionWriterV1;
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
    /// Row-major table cell expressions.
    Table(#[serde(serialize_with = "serialize_table_rows")] Vec<Vec<ShareableString>>),
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
            MapItemFrozen::Table(value) => Self::Table(value.rows().to_vec()),
            MapItemFrozen::TableWithUnits(value) => Self::TableWithUnits {
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
    use crate::common::deserialize::RootObject;
    use crate::datastore::frozen::reader::v1::map::MapReaderV1;
    use serde_json::json;

    #[test]
    fn writes_values_without_repeating_item_definitions() {
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
                    }
                ],
                "default": null
            },
            "value": {
                "first": [
                    "sample",
                    {"value": "12 m", "units": "length_meter"},
                    {"units": ["length_meter"], "rows": [["12 m"]]}
                ]
            }
        });
        let reader = serde_json::from_value::<RootObject<MapReaderV1>>(document.clone())
            .expect("valid map")
            .document;
        let map = MapFrozen::try_from(reader).expect("matching map");

        assert_eq!(
            serde_json::to_value(MapWriterV1::from(&map)).unwrap(),
            document
        );
    }
}
