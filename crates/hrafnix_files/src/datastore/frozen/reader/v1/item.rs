use super::{
    boolean::BooleanReaderV1, choice::ChoiceReaderV1, file::FileReaderV1, folder::FolderReaderV1,
    integer::IntegerReaderV1, map::MapReaderV1, number::NumberReaderV1,
    number_with_units::NumberWithUnitsReaderV1, separator::SeparatorReaderV1,
    string::StringReaderV1, tab::TabReaderV1, table::TableReaderV1,
    table_with_units::TableWithUnitsReaderV1, unit::UnitReaderV1,
};
use crate::common::deserialize::StringTagDeserializer;
use hrafnix_datastore::frozen::ItemFrozen;
use hrafnix_message::message::Message;
use serde::{Deserialize, Deserializer};

/// Version-1 frozen item with a string type tag and its embedded definition.
#[derive(Debug, Deserialize)]
#[serde(remote = "Self", tag = "type", rename_all = "snake_case")]
pub(crate) enum ItemReaderV1 {
    /// Boolean definition and expression.
    Boolean(BooleanReaderV1),
    /// Choice definition and expression.
    Choice(ChoiceReaderV1),
    /// File definition and path expression.
    File(FileReaderV1),
    /// Folder definition and path expression.
    Folder(FolderReaderV1),
    /// Integer definition and expression.
    Integer(IntegerReaderV1),
    /// Map definition and entry values.
    Map(MapReaderV1),
    /// Number definition and expression.
    Number(NumberReaderV1),
    /// Number definition, expression, and selected units.
    NumberWithUnits(NumberWithUnitsReaderV1),
    /// String definition and expression.
    String(StringReaderV1),
    /// Table definition and cell expressions.
    Table(TableReaderV1),
    /// Table definition, cell expressions, and column units.
    TableWithUnits(TableWithUnitsReaderV1),
    /// Unit definition and selection expression.
    Unit(UnitReaderV1),
    /// Tab metadata without a value.
    Tab(TabReaderV1),
    /// Separator metadata without a value.
    Separator(SeparatorReaderV1),
}

impl<'de> Deserialize<'de> for ItemReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::deserialize(StringTagDeserializer(deserializer))
    }
}

impl TryFrom<ItemReaderV1> for ItemFrozen {
    type Error = Message;

    fn try_from(item: ItemReaderV1) -> Result<Self, Self::Error> {
        Ok(match item {
            ItemReaderV1::Boolean(value) => Self::Boolean(value.into()),
            ItemReaderV1::Choice(value) => Self::Choice(value.into()),
            ItemReaderV1::File(value) => Self::File(value.into()),
            ItemReaderV1::Folder(value) => Self::Folder(value.into()),
            ItemReaderV1::Integer(value) => Self::Integer(value.into()),
            ItemReaderV1::Map(value) => Self::Map(value.try_into()?),
            ItemReaderV1::Number(value) => Self::Number(value.into()),
            ItemReaderV1::NumberWithUnits(value) => Self::NumberWithUnits(value.into()),
            ItemReaderV1::String(value) => Self::String(value.into()),
            ItemReaderV1::Table(value) => Self::Table(value.into()),
            ItemReaderV1::TableWithUnits(value) => Self::TableWithUnits(value.into()),
            ItemReaderV1::Unit(value) => Self::Unit(value.into()),
            ItemReaderV1::Tab(value) => Self::Tab(value.into()),
            ItemReaderV1::Separator(value) => Self::Separator(value.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use crate::datastore::frozen::writer::v1::item::ItemWriterV1;
    use hrafnix_datastore::frozen::MapItemFrozen;
    use serde_json::{Value, json};

    fn table_json(with_units: bool, column_count: usize, parameter: &str) -> Value {
        let columns: Vec<_> = (0..column_count)
            .map(|index| {
                let mut column = json!({
                    "key": format!("column_{index}"),
                    "description": "",
                    "default": "0",
                    "constraint": {"type": "none"},
                    "requirement": "none"
                });
                if with_units {
                    column["preferred_unit"] = json!("length_meter");
                }
                column
            })
            .collect();
        let mut table = json!({
            "type": if with_units { "table_with_units" } else { "table" },
            "definition": {"description": "", "columns": columns, "default": null},
            "parameter": parameter,
            "rows": [vec!["7"; column_count]]
        });
        if with_units {
            table["units"] = json!(vec!["length_meter"; column_count]);
        }
        table
    }

    fn map_with_tables_json(column_count: usize, parameter: &str) -> Value {
        let mut definitions = Vec::new();
        let mut values = Vec::new();
        for with_units in [false, true] {
            let mut table = table_json(with_units, column_count, parameter);
            let fields = table.as_object_mut().unwrap();
            let mut definition = fields.remove("definition").unwrap();
            definition["type"] = fields.remove("type").unwrap();
            definition["key"] = json!(if with_units { "units" } else { "plain" });
            definitions.push(definition);
            values.push(table);
        }
        json!({
            "type": "map",
            "definition": {"description": "", "items": definitions, "default": null},
            "value": {"entry": values}
        })
    }

    fn read_item(document: Value) -> Result<ItemFrozen, Message> {
        let reader = serde_json::from_value::<RootObject<ItemReaderV1>>(document)
            .unwrap()
            .document;
        reader.try_into()
    }

    #[test]
    fn tables_preserve_references_rows_and_column_units() {
        for with_units in [false, true] {
            for column_count in [0, 1, 2] {
                for parameter in ["", "p_source"] {
                    let document = table_json(with_units, column_count, parameter);
                    let item = read_item(document.clone()).unwrap();
                    if with_units {
                        let ItemFrozen::TableWithUnits(table) = &item else {
                            panic!("expected a table with units");
                        };
                        assert_eq!(table.parameter(), parameter);
                        let mut editable = table.thaw();
                        for index in 0..column_count {
                            editable
                                .set_unit_by_name(format!("column_{index}"), "length_centimeter")
                                .unwrap();
                        }
                    } else {
                        let ItemFrozen::Table(table) = &item else {
                            panic!("expected a table");
                        };
                        assert_eq!(table.parameter(), parameter);
                    }
                    let written = serde_json::to_value(ItemWriterV1::from(&item)).unwrap();
                    assert_eq!(written, document);
                    assert_eq!(read_item(written).unwrap(), item);
                }
            }
        }
    }

    #[test]
    fn map_tables_preserve_references_rows_and_column_units() {
        for column_count in [0, 1, 2] {
            for parameter in ["", "p_source"] {
                let document = map_with_tables_json(column_count, parameter);
                let item = read_item(document.clone()).unwrap();
                let ItemFrozen::Map(map) = &item else {
                    panic!("expected a map");
                };
                let entry = map.get("entry").unwrap();
                let Some(MapItemFrozen::Table(plain)) = entry.get("plain") else {
                    panic!("expected a table");
                };
                assert_eq!(plain.parameter(), parameter);
                let Some(MapItemFrozen::TableWithUnits(units)) = entry.get("units") else {
                    panic!("expected a table with units");
                };
                assert_eq!(units.parameter(), parameter);
                let mut editable = units.thaw();
                for index in 0..column_count {
                    editable
                        .set_unit_by_name(format!("column_{index}"), "length_centimeter")
                        .unwrap();
                }
                let written = serde_json::to_value(ItemWriterV1::from(&item)).unwrap();
                assert_eq!(written, document);
                assert_eq!(read_item(written).unwrap(), item);
            }
        }
    }

    #[test]
    fn rejects_missing_or_extra_column_units_in_tables_and_maps() {
        for column_count in [0, 1, 2] {
            for unit_count in [0, 1, 2, 3] {
                if unit_count == column_count {
                    continue;
                }
                let units = json!(vec!["length_meter"; unit_count]);
                let mut table = table_json(true, column_count, "");
                table["units"] = units.clone();
                let error = serde_json::from_value::<RootObject<ItemReaderV1>>(table).unwrap_err();
                assert!(
                    error.to_string().contains(&format!(
                        "table has {unit_count} units but {column_count} columns"
                    )),
                    "{error}"
                );

                let mut map = map_with_tables_json(column_count, "");
                map["value"]["entry"][1]["units"] = units;
                let error = read_item(map).unwrap_err();
                assert_eq!(
                    error.translate_data().message_key().as_str(),
                    "datastore_schema_mismatch"
                );
            }
        }
    }

    #[test]
    fn tables_require_explicit_string_references_and_map_objects() {
        for with_units in [false, true] {
            let table = table_json(with_units, 1, "");
            let map = map_with_tables_json(1, "");
            let index = usize::from(with_units);
            for parameter in [
                None,
                Some(Value::Null),
                Some(json!(1)),
                Some(json!(false)),
                Some(json!([])),
                Some(json!({})),
            ] {
                let mut invalid_table = table.clone();
                let mut invalid_map = map.clone();
                for value in [
                    &mut invalid_table,
                    &mut invalid_map["value"]["entry"][index],
                ] {
                    if let Some(parameter) = &parameter {
                        value["parameter"] = parameter.clone();
                    } else {
                        value.as_object_mut().unwrap().remove("parameter");
                    }
                }
                for invalid in [invalid_table, invalid_map] {
                    assert!(serde_json::from_value::<RootObject<ItemReaderV1>>(invalid).is_err());
                }
            }
            for invalid_value in [
                json!([["7"]]),
                json!(["p_source", [["7"]]]),
                json!(["p_source", ["length_meter"], [["7"]]]),
            ] {
                let mut invalid_map = map.clone();
                invalid_map["value"]["entry"][index] = invalid_value;
                assert!(serde_json::from_value::<RootObject<ItemReaderV1>>(invalid_map).is_err());
            }
        }
    }

    #[test]
    fn reads_tagged_frozen_item() {
        let value = json!({
            "type": "boolean",
            "definition": {
                "description": "Enabled",
                "default": "true",
                "true_description": "On",
                "false_description": "Off"
            },
            "value": "false"
        });
        let reader = serde_json::from_value::<RootObject<ItemReaderV1>>(value.clone())
            .unwrap()
            .document;
        let item = ItemFrozen::try_from(reader).unwrap();

        assert_eq!(
            serde_json::to_value(ItemWriterV1::from(&item)).unwrap(),
            value
        );
    }

    #[test]
    fn reads_map_through_fallible_conversion() {
        let value = json!({
            "type": "map",
            "definition": {
                "description": "Labels",
                "items": [
                    {"key": "name", "type": "string", "description": "", "default": ""}
                ],
                "default": null
            },
            "value": {"first": ["sample"]}
        });
        let reader = serde_json::from_value::<RootObject<ItemReaderV1>>(value.clone())
            .unwrap()
            .document;
        let item = ItemFrozen::try_from(reader).unwrap();

        assert_eq!(
            serde_json::to_value(ItemWriterV1::from(&item)).unwrap(),
            value
        );
    }

    #[test]
    fn rejects_non_string_tags() {
        assert!(
            serde_json::from_value::<RootObject<ItemReaderV1>>(json!({
                "type": 0,
                "definition": {
                    "description": "",
                    "default": ""
                },
                "value": ""
            }))
            .is_err()
        );
    }
}
