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
    use serde_json::json;

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
