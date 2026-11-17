use super::{
    boolean::BooleanReaderV1, choice::ChoiceReaderV1, file::FileReaderV1, folder::FolderReaderV1,
    integer::IntegerReaderV1, map::MapReaderV1, number::NumberReaderV1,
    number_with_units::NumberWithUnitReaderV1, separator::SeparatorReaderV1,
    string::StringReaderV1, tab::TabReaderV1, table::TableReaderV1,
    table_with_units::TableWithUnitsReaderV1, unit::UnitReaderV1,
};
use crate::common::deserialize::StringTagDeserializer;
use hrafnix_datastore::definition::ItemDefinitionType;
use serde::{Deserialize, Deserializer};

/// Version-1 tagged item definition, flattened alongside its typed key.
#[derive(Debug, Deserialize)]
#[serde(remote = "Self", tag = "type", rename_all = "snake_case")]
pub(crate) enum ItemDefinitionReaderV1 {
    /// Boolean expression and state labels.
    Boolean(BooleanReaderV1),
    /// Choice expression and choices.
    Choice(ChoiceReaderV1),
    /// File expression and dialog options.
    File(FileReaderV1),
    /// Folder expression and archive option.
    Folder(FolderReaderV1),
    /// Integer expression and bounds.
    Integer(IntegerReaderV1),
    /// Ordered map schema and default entries.
    Map(MapReaderV1),
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
    /// Tab label.
    Tab(TabReaderV1),
    /// Separator label.
    Separator(SeparatorReaderV1),
}

impl<'de> Deserialize<'de> for ItemDefinitionReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::deserialize(StringTagDeserializer(deserializer))
    }
}

impl From<ItemDefinitionReaderV1> for ItemDefinitionType {
    fn from(definition: ItemDefinitionReaderV1) -> Self {
        match definition {
            ItemDefinitionReaderV1::Boolean(item) => Self::Boolean(item.into()),
            ItemDefinitionReaderV1::Choice(item) => Self::Choice(item.into()),
            ItemDefinitionReaderV1::File(item) => Self::File(item.into()),
            ItemDefinitionReaderV1::Folder(item) => Self::Folder(item.into()),
            ItemDefinitionReaderV1::Integer(item) => Self::Integer(item.into()),
            ItemDefinitionReaderV1::Map(item) => Self::Map(item.into()),
            ItemDefinitionReaderV1::Number(item) => Self::Number(item.into()),
            ItemDefinitionReaderV1::NumberWithUnits(item) => Self::NumberWithUnits(item.into()),
            ItemDefinitionReaderV1::String(item) => Self::String(item.into()),
            ItemDefinitionReaderV1::Table(item) => Self::Table(item.into()),
            ItemDefinitionReaderV1::TableWithUnits(item) => Self::TableWithUnits(item.into()),
            ItemDefinitionReaderV1::Unit(item) => Self::Unit(item.into()),
            ItemDefinitionReaderV1::Tab(item) => Self::Tab(item.into()),
            ItemDefinitionReaderV1::Separator(item) => Self::Separator(item.into()),
        }
    }
}
