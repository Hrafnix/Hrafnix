use super::{
    boolean::BooleanDefinitionReaderV1, choice::ChoiceDefinitionReaderV1,
    file::FileDefinitionReaderV1, folder::FolderDefinitionReaderV1,
    integer::IntegerDefinitionReaderV1, map::MapDefinitionReaderV1,
    number::NumberDefinitionReaderV1, number_with_units::NumberWithUnitDefinitionReaderV1,
    separator::SeparatorDefinitionReaderV1, string::StringDefinitionReaderV1,
    tab::TabDefinitionReaderV1, table::TableDefinitionReaderV1,
    table_with_units::TableWithUnitsDefinitionReaderV1, unit::UnitDefinitionReaderV1,
};
use crate::common::deserialize::StringTagDeserializer;
use hrafnix_datastore::definition::ItemDefinitionType;
use serde::{Deserialize, Deserializer};

/// Version-1 tagged item definition, flattened alongside its typed key.
#[derive(Debug, Deserialize)]
#[serde(remote = "Self", tag = "type", rename_all = "snake_case")]
pub(crate) enum ItemDefinitionReaderV1 {
    /// Boolean expression and state labels.
    Boolean(BooleanDefinitionReaderV1),
    /// Choice expression and choices.
    Choice(ChoiceDefinitionReaderV1),
    /// File expression and dialog options.
    File(FileDefinitionReaderV1),
    /// Folder expression and archive option.
    Folder(FolderDefinitionReaderV1),
    /// Integer expression and bounds.
    Integer(IntegerDefinitionReaderV1),
    /// Ordered map schema and default entries.
    Map(MapDefinitionReaderV1),
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
    /// Tab label.
    Tab(TabDefinitionReaderV1),
    /// Separator label.
    Separator(SeparatorDefinitionReaderV1),
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
