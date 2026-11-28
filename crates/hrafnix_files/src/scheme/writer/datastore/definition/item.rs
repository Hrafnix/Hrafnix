use super::{
    boolean::BooleanDefinitionWriterV1, choice::ChoiceDefinitionWriterV1,
    file::FileDefinitionWriterV1, folder::FolderDefinitionWriterV1,
    integer::IntegerDefinitionWriterV1, map::MapDefinitionWriterV1,
    number::NumberDefinitionWriterV1, number_with_units::NumberWithUnitDefinitionWriterV1,
    separator::SeparatorDefinitionWriterV1, string::StringDefinitionWriterV1,
    tab::TabDefinitionWriterV1, table::TableDefinitionWriterV1,
    table_with_units::TableWithUnitsDefinitionWriterV1, unit::UnitDefinitionWriterV1,
};
use hrafnix_datastore::definition::ItemDefinitionType;
use serde::Serialize;

/// Current tagged item definition, flattened alongside its typed key.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum ItemDefinitionWriter {
    /// Boolean expression and state labels.
    Boolean(BooleanDefinitionWriterV1),
    /// Choice expression and choices.
    Choice(ChoiceDefinitionWriterV1),
    /// File expression and dialog options.
    File(FileDefinitionWriterV1),
    /// Folder expression and archive option.
    Folder(FolderDefinitionWriterV1),
    /// Integer expression and bounds.
    Integer(IntegerDefinitionWriterV1),
    /// Ordered map schema and default entries.
    Map(MapDefinitionWriterV1),
    /// Numeric expression and bounds.
    Number(NumberDefinitionWriterV1),
    /// Numeric expression with units.
    NumberWithUnits(NumberWithUnitDefinitionWriterV1),
    /// String expression.
    String(StringDefinitionWriterV1),
    /// Numeric table.
    Table(TableDefinitionWriterV1),
    /// Numeric table with units.
    TableWithUnits(TableWithUnitsDefinitionWriterV1),
    /// Unit selection.
    Unit(UnitDefinitionWriterV1),
    /// Tab label.
    Tab(TabDefinitionWriterV1),
    /// Separator label.
    Separator(SeparatorDefinitionWriterV1),
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
