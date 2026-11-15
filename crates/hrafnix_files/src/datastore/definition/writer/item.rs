use super::{
    boolean::BooleanWriterV1, choice::ChoiceWriterV1, file::FileWriterV1, folder::FolderWriterV1,
    integer::IntegerWriterV1, map::MapWriterV1, number::NumberWriterV1,
    number_with_units::NumberWithUnitWriterV1, separator::SeparatorWriterV1,
    string::StringWriterV1, tab::TabWriterV1, table::TableWriterV1,
    table_with_units::TableWithUnitsWriterV1, unit::UnitWriterV1,
};
use hrafnix_datastore::definition::ItemDefinitionType;
use serde::Serialize;

/// Current tagged item definition, flattened alongside its typed key.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum ItemDefinitionWriter {
    /// Boolean expression and state labels.
    Boolean(BooleanWriterV1),
    /// Choice expression and choices.
    Choice(ChoiceWriterV1),
    /// File expression and dialog options.
    File(FileWriterV1),
    /// Folder expression and archive option.
    Folder(FolderWriterV1),
    /// Integer expression and bounds.
    Integer(IntegerWriterV1),
    /// Ordered map schema and default entries.
    Map(MapWriterV1),
    /// Numeric expression and bounds.
    Number(NumberWriterV1),
    /// Numeric expression with units.
    NumberWithUnits(NumberWithUnitWriterV1),
    /// String expression.
    String(StringWriterV1),
    /// Numeric table.
    Table(TableWriterV1),
    /// Numeric table with units.
    TableWithUnits(TableWithUnitsWriterV1),
    /// Unit selection.
    Unit(UnitWriterV1),
    /// Tab label.
    Tab(TabWriterV1),
    /// Separator label.
    Separator(SeparatorWriterV1),
}

impl From<&ItemDefinitionType> for ItemDefinitionWriter {
    fn from(definition: &ItemDefinitionType) -> Self {
        match definition {
            ItemDefinitionType::Boolean(item) => Self::Boolean(item.into()),
            ItemDefinitionType::Choice(item) => Self::Choice(item.into()),
            ItemDefinitionType::File(item) => Self::File(item.into()),
            ItemDefinitionType::Folder(item) => Self::Folder(item.into()),
            ItemDefinitionType::Integer(item) => Self::Integer(item.into()),
            ItemDefinitionType::Map(item) => Self::Map(item.into()),
            ItemDefinitionType::Number(item) => Self::Number(item.into()),
            ItemDefinitionType::NumberWithUnits(item) => Self::NumberWithUnits(item.into()),
            ItemDefinitionType::String(item) => Self::String(item.into()),
            ItemDefinitionType::Table(item) => Self::Table(item.into()),
            ItemDefinitionType::TableWithUnits(item) => Self::TableWithUnits(item.into()),
            ItemDefinitionType::Unit(item) => Self::Unit(item.into()),
            ItemDefinitionType::Tab(item) => Self::Tab(item.into()),
            ItemDefinitionType::Separator(item) => Self::Separator(item.into()),
        }
    }
}
