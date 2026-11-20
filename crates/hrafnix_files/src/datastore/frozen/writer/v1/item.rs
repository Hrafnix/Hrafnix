use super::{
    boolean::BooleanWriterV1, choice::ChoiceWriterV1, file::FileWriterV1, folder::FolderWriterV1,
    integer::IntegerWriterV1, map::MapWriterV1, number::NumberWriterV1,
    number_with_units::NumberWithUnitsWriterV1, separator::SeparatorWriterV1,
    string::StringWriterV1, tab::TabWriterV1, table::TableWriterV1,
    table_with_units::TableWithUnitsWriterV1, unit::UnitWriterV1,
};
use hrafnix_datastore::frozen::ItemFrozen;
use serde::Serialize;

/// Version-1 frozen item with a string type tag and its embedded definition.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum ItemWriterV1 {
    /// Boolean definition and expression.
    Boolean(BooleanWriterV1),
    /// Choice definition and expression.
    Choice(ChoiceWriterV1),
    /// File definition and path expression.
    File(FileWriterV1),
    /// Folder definition and path expression.
    Folder(FolderWriterV1),
    /// Integer definition and expression.
    Integer(IntegerWriterV1),
    /// Map definition and entry values.
    Map(MapWriterV1),
    /// Number definition and expression.
    Number(NumberWriterV1),
    /// Number definition, expression, and selected units.
    NumberWithUnits(NumberWithUnitsWriterV1),
    /// String definition and expression.
    String(StringWriterV1),
    /// Table definition and cell expressions.
    Table(TableWriterV1),
    /// Table definition, cell expressions, and column units.
    TableWithUnits(TableWithUnitsWriterV1),
    /// Unit definition and selection expression.
    Unit(UnitWriterV1),
    /// Tab metadata without a value.
    Tab(TabWriterV1),
    /// Separator metadata without a value.
    Separator(SeparatorWriterV1),
}

impl From<&ItemFrozen> for ItemWriterV1 {
    fn from(item: &ItemFrozen) -> Self {
        match item {
            ItemFrozen::Boolean(value) => Self::Boolean(value.into()),
            ItemFrozen::Choice(value) => Self::Choice(value.into()),
            ItemFrozen::File(value) => Self::File(value.into()),
            ItemFrozen::Folder(value) => Self::Folder(value.into()),
            ItemFrozen::Integer(value) => Self::Integer(value.into()),
            ItemFrozen::Map(value) => Self::Map(value.into()),
            ItemFrozen::Number(value) => Self::Number(value.into()),
            ItemFrozen::NumberWithUnits(value) => Self::NumberWithUnits(value.into()),
            ItemFrozen::String(value) => Self::String(value.into()),
            ItemFrozen::Table(value) => Self::Table(value.into()),
            ItemFrozen::TableWithUnits(value) => Self::TableWithUnits(value.into()),
            ItemFrozen::Unit(value) => Self::Unit(value.into()),
            ItemFrozen::Tab(value) => Self::Tab(value.into()),
            ItemFrozen::Separator(value) => Self::Separator(value.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::BooleanDefinition;
    use hrafnix_datastore::frozen::BooleanFrozen;
    use serde_json::json;

    #[test]
    fn writes_tagged_frozen_item() {
        let item = ItemFrozen::Boolean(BooleanFrozen::new_with_value(
            BooleanDefinition::new_with_descriptions_and_default("Enabled", "On", "Off", "true"),
            "false",
        ));

        assert_eq!(
            serde_json::to_value(ItemWriterV1::from(&item)).unwrap(),
            json!({
                "type": "boolean",
                "definition": {
                    "description": "Enabled",
                    "default": "true",
                    "true_description": "On",
                    "false_description": "Off"
                },
                "value": "false"
            })
        );
    }
}
