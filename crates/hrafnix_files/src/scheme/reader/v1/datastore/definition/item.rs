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
use hrafnix_message::message::Message;
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

impl TryFrom<ItemDefinitionReaderV1> for ItemDefinitionType {
    type Error = Message;

    fn try_from(definition: ItemDefinitionReaderV1) -> Result<Self, Self::Error> {
        Ok(match definition {
            ItemDefinitionReaderV1::Boolean(item) => Self::Boolean(item.into()),
            ItemDefinitionReaderV1::Choice(item) => Self::Choice(item.try_into()?),
            ItemDefinitionReaderV1::File(item) => Self::File(item.into()),
            ItemDefinitionReaderV1::Folder(item) => Self::Folder(item.into()),
            ItemDefinitionReaderV1::Integer(item) => Self::Integer(item.into()),
            ItemDefinitionReaderV1::Map(item) => Self::Map(item.try_into()?),
            ItemDefinitionReaderV1::Number(item) => Self::Number(item.into()),
            ItemDefinitionReaderV1::NumberWithUnits(item) => Self::NumberWithUnits(item.into()),
            ItemDefinitionReaderV1::String(item) => Self::String(item.into()),
            ItemDefinitionReaderV1::Table(item) => Self::Table(item.try_into()?),
            ItemDefinitionReaderV1::TableWithUnits(item) => Self::TableWithUnits(item.try_into()?),
            ItemDefinitionReaderV1::Unit(item) => Self::Unit(item.into()),
            ItemDefinitionReaderV1::Tab(item) => Self::Tab(item.into()),
            ItemDefinitionReaderV1::Separator(item) => Self::Separator(item.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::{
        BooleanDefinition, ChoiceDefinition, ChoiceItemDefinition, FileDefinition,
        FolderDefinition, IntegerConstraint, IntegerDefinition, MapDefinition, MapItemDefault,
        MapItemDefinition, NumberConstraint, NumberDefinition, NumberWithUnitsDefinition,
        SeparatorDefinition, StringDefinition, TabDefinition, TableColumnRequirement,
        TableDefinition, TableWithUnitsDefinition, UnitDefinition,
    };
    use hrafnix_keys::StoreKey;
    use hrafnix_units::{UnitFamilyId, UnitId};
    use serde_json::{Value, json};
    use std::collections::BTreeMap;

    /// Definition payloads and their expected conversion values.
    fn definition_cases() -> Vec<(Value, ItemDefinitionType)> {
        let key = |name: &str| StoreKey::new(name.into()).unwrap();
        vec![
            (
                json!({
                    "type": "boolean", "description": "Flag", "default": "1 == 1",
                    "true_description": "On", "false_description": "Off"
                }),
                BooleanDefinition::new_with_descriptions_and_default("Flag", "On", "Off", "1 == 1")
                    .into(),
            ),
            (
                json!({
                    "type": "choice", "description": "Mode", "default": "fast",
                    "items": [{"id": "fast", "description": "Fast"}]
                }),
                ChoiceDefinition::new_with_default(
                    "Mode",
                    vec![ChoiceItemDefinition::new(key("fast"), "Fast")],
                    "fast",
                )
                .into(),
            ),
            (
                json!({
                    "type": "file", "description": "Input", "default": "input.csv",
                    "extension_filter": "*.csv", "is_input": true
                }),
                FileDefinition::new_with_default("Input", "*.csv", true, "input.csv").into(),
            ),
            (
                json!({
                    "type": "folder", "description": "Workspace",
                    "default": "data/input", "is_input": false
                }),
                FolderDefinition::new_with_default("Workspace", false, "data/input").into(),
            ),
            (
                json!({
                    "type": "integer", "description": "Count", "default": "2 + 3",
                    "constraint": {"type": "min", "min": 0, "inclusive": true}
                }),
                IntegerDefinition::new_with_constraint_and_default(
                    "Count",
                    IntegerConstraint::min(0, true),
                    "2 + 3",
                )
                .into(),
            ),
            (
                json!({
                    "type": "map", "description": "Labels",
                    "items": [{
                        "key": "label", "type": "string", "description": "Label", "default": "base"
                    }],
                    "default": {"first": ["custom"]}
                }),
                MapDefinition::new_with_default(
                    "Labels",
                    vec![(
                        key("label"),
                        MapItemDefinition::String(StringDefinition::new_with_default(
                            "Label", "base",
                        )),
                    )],
                    BTreeMap::from([(key("first"), vec![MapItemDefault::scalar("custom")])]),
                )
                .into(),
            ),
            (
                json!({
                    "type": "number", "description": "Ratio", "default": "1 / 2",
                    "constraint": {"type": "max", "max": 2.5, "inclusive": false}
                }),
                NumberDefinition::new_with_constraint_and_default(
                    "Ratio",
                    NumberConstraint::max(2.5, false),
                    "1 / 2",
                )
                .into(),
            ),
            (
                json!({
                    "type": "number_with_units", "description": "Distance", "default": "12 cm",
                    "constraint": {"type": "min", "min": 0.0, "inclusive": true},
                    "preferred_unit": "length_centimeter"
                }),
                NumberWithUnitsDefinition::new_with_constraint_and_default(
                    "Distance",
                    NumberConstraint::min(0.0, true),
                    "12 cm",
                    UnitId::Length_Centimeter,
                )
                .into(),
            ),
            (
                json!({"type": "string", "description": "Label", "default": "héllo"}),
                StringDefinition::new_with_default("Label", "héllo").into(),
            ),
            (
                json!({
                    "type": "table", "description": "Samples",
                    "columns": [{
                        "key": "value", "description": "Value", "default": "1",
                        "constraint": {"type": "none"}, "requirement": "increasing"
                    }],
                    "default": [["2"], ["3"]]
                }),
                TableDefinition::new_with_column_requirements_and_default(
                    "Samples",
                    vec![(
                        key("value"),
                        NumberDefinition::new_with_default("Value", "1"),
                        TableColumnRequirement::Increasing,
                    )],
                    vec![vec!["2"], vec!["3"]],
                )
                .into(),
            ),
            (
                json!({
                    "type": "table_with_units", "description": "Positions",
                    "columns": [{
                        "key": "distance", "description": "Distance", "default": "1 m",
                        "constraint": {"type": "none"}, "requirement": "decreasing",
                        "preferred_unit": "length_meter"
                    }],
                    "default": [["3 m"], ["2 m"]]
                }),
                TableWithUnitsDefinition::new_with_column_requirements_and_default(
                    "Positions",
                    vec![(
                        key("distance"),
                        NumberWithUnitsDefinition::new_with_default(
                            "Distance",
                            "1 m",
                            UnitId::Length_Meter,
                        ),
                        TableColumnRequirement::Decreasing,
                    )],
                    vec![vec!["3 m"], vec!["2 m"]],
                )
                .into(),
            ),
            (
                json!({
                    "type": "unit", "description": "Units",
                    "default": "u_length_meter", "unit_family": "length"
                }),
                UnitDefinition::new_with_default("Units", UnitFamilyId::Length, "u_length_meter")
                    .into(),
            ),
            (
                json!({"type": "tab", "description": "Advanced"}),
                TabDefinition::new("Advanced").into(),
            ),
            (
                json!({"type": "separator", "description": "Outputs"}),
                SeparatorDefinition::new("Outputs").into(),
            ),
        ]
    }

    #[test]
    fn dispatches_every_definition_type_and_preserves_data() {
        let cases = definition_cases();
        assert_eq!(cases.len(), 14);
        for (value, expected) in cases {
            let reader =
                serde_json::from_value::<RootObject<ItemDefinitionReaderV1>>(value.clone())
                    .unwrap();
            assert_eq!(
                ItemDefinitionType::try_from(reader.document).unwrap(),
                expected,
                "{value}"
            );
        }
    }

    #[test]
    fn rejects_invalid_type_tags() {
        for tag in [
            json!("unknown"),
            json!(1),
            json!({"string": null}),
            Value::Null,
        ] {
            assert!(
                serde_json::from_value::<RootObject<ItemDefinitionReaderV1>>(json!({
                    "type": tag, "description": "Label", "default": "text"
                }))
                .is_err()
            );
        }
    }

    #[test]
    fn propagates_fallible_definition_validation() {
        let reader = serde_json::from_value::<RootObject<ItemDefinitionReaderV1>>(json!({
            "type": "table", "description": "Samples",
            "columns": [{
                "key": "value", "description": "Value", "default": "1",
                "constraint": {"type": "none"}, "requirement": "none"
            }],
            "default": [["2", "3"]]
        }))
        .unwrap();
        let error = ItemDefinitionType::try_from(reader.document).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_table_default_row_width"
        );
    }
}
