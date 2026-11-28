//! Public file-API round trips for ordered definitions, exact bounds, and canonical defaults.

#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::unwrap_used)]

use super::common::{self, TestDirectory};

use camino::Utf8Path;
use hrafnix_datastore::definition::{
    BooleanDefinition, ChoiceDefinition, ChoiceItemDefinition, FileDefinition, FolderDefinition,
    IntegerConstraint, IntegerDefinition, ItemDefinitionType, MapDefinition, MapItemDefault,
    MapItemDefinition, NumberConstraint, NumberDefinition, NumberWithUnitsDefinition,
    ParameterObjectDefinition, SeparatorDefinition, StringDefinition, TabDefinition,
    TableColumnRequirement, TableDefinition, TableWithUnitsDefinition, UnitDefinition,
    VariableObjectDefinition,
};
use hrafnix_files::formats::{
    component::{read_component_file, write_component_file},
    model::{read_model_file, write_model_file},
};
use hrafnix_keys::{ParameterKey, StoreKey, VariableKey};
use hrafnix_units::{UnitFamilyId, UnitId};
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs};

/// Creates a validated nested item or column key.
fn key(value: &str) -> StoreKey {
    StoreKey::new(value.into()).unwrap()
}

/// Independent datastore definitions and their exact unkeyed JSON representation.
fn item_cases() -> Vec<(&'static str, ItemDefinitionType, Value)> {
    vec![
        (
            "z_boolean",
            BooleanDefinition::new_with_descriptions_and_default("Flag", "On", "Off", "1 == 1")
                .into(),
            json!({
                "type": "boolean", "description": "Flag", "default": "1 == 1",
                "true_description": "On", "false_description": "Off"
            }),
        ),
        (
            "y_choice",
            ChoiceDefinition::new_with_default(
                "Mode",
                vec![
                    ChoiceItemDefinition::new(key("z"), "First"),
                    ChoiceItemDefinition::new(key("a"), "Second"),
                ],
                "z",
            )
            .into(),
            json!({
                "type": "choice", "description": "Mode", "default": "z",
                "items": [{"id": "z", "description": "First"}, {"id": "a", "description": "Second"}]
            }),
        ),
        (
            "x_file",
            FileDefinition::new_with_default("File", "*.csv;*.txt", true, "../input.csv").into(),
            json!({
                "type": "file", "description": "File", "default": "../input.csv",
                "extension_filter": "*.csv;*.txt", "is_input": true
            }),
        ),
        (
            "w_folder",
            FolderDefinition::new_with_default("Folder", false, r"C:\missing folder\").into(),
            json!({
                "type": "folder", "description": "Folder", "default": r"C:\missing folder\",
                "is_input": false
            }),
        ),
        (
            "v_integer",
            IntegerDefinition::new_with_constraint_and_default(
                "Count",
                IntegerConstraint::range(i64::MIN, i64::MAX, false, true),
                "2 + 3",
            )
            .into(),
            json!({
                "type": "integer", "description": "Count", "default": "2 + 3",
                "constraint": {
                    "type": "range", "min": i64::MIN, "max": i64::MAX,
                    "min_inclusive": false, "max_inclusive": true
                }
            }),
        ),
        (
            "u_map",
            MapDefinition::new_with_default(
                "Map",
                vec![
                    (
                        key("z"),
                        MapItemDefinition::from(StringDefinition::new_with_default(
                            "Label", "base",
                        )),
                    ),
                    (
                        key("a"),
                        TableDefinition::new(
                            "Rows",
                            vec![(key("value"), NumberDefinition::new(""))],
                        )
                        .into(),
                    ),
                    (
                        key("b"),
                        TableWithUnitsDefinition::new(
                            "Distances",
                            vec![(
                                key("distance"),
                                NumberWithUnitsDefinition::new("", UnitId::Length_Meter),
                            )],
                        )
                        .into(),
                    ),
                ],
                BTreeMap::from([(
                    key("entry"),
                    vec![
                        MapItemDefault::scalar("custom"),
                        MapItemDefault::table(vec![vec!["1"], vec!["2"]]),
                        MapItemDefault::table(vec![vec!["3 m"]]),
                    ],
                )]),
            )
            .into(),
            json!({
                "type": "map", "description": "Map",
                "items": [
                    {"key": "z", "type": "string", "description": "Label", "default": "base"},
                    {
                        "key": "a", "type": "table", "description": "Rows",
                        "columns": [{"key": "value", "description": "", "default": "", "constraint": {"type": "none"}, "requirement": "none"}],
                        "default": null
                    },
                    {
                        "key": "b", "type": "table_with_units", "description": "Distances",
                        "columns": [{"key": "distance", "description": "", "default": "", "constraint": {"type": "none"}, "requirement": "none", "preferred_unit": "length_meter"}],
                        "default": null
                    }
                ],
                "default": {"entry": ["custom", [["1"], ["2"]], [["3 m"]]]}
            }),
        ),
        (
            "t_number",
            NumberDefinition::new_with_constraint_and_default(
                "Value",
                NumberConstraint::min(-1.5, true),
                "2.5 * factor",
            )
            .into(),
            json!({
                "type": "number", "description": "Value", "default": "2.5 * factor",
                "constraint": {"type": "min", "min": -1.5, "inclusive": true}
            }),
        ),
        (
            "s_number_with_units",
            NumberWithUnitsDefinition::new_with_constraint_and_default(
                "Distance",
                NumberConstraint::max(10.0, false),
                "2 m",
                UnitId::Length_Meter,
            )
            .into(),
            json!({
                "type": "number_with_units", "description": "Distance", "default": "2 m",
                "constraint": {"type": "max", "max": 10.0, "inclusive": false},
                "preferred_unit": "length_meter"
            }),
        ),
        (
            "r_string",
            StringDefinition::new_with_default("Label \u{3b1}", " \"quoted\"\n ").into(),
            json!({"type": "string", "description": "Label \u{3b1}", "default": " \"quoted\"\n "}),
        ),
        (
            "q_table",
            TableDefinition::new_with_column_requirements_and_default(
                "Table",
                vec![
                    (
                        key("z"),
                        NumberDefinition::new_with_default("First", "1"),
                        TableColumnRequirement::Increasing,
                    ),
                    (
                        key("a"),
                        NumberDefinition::new_with_default("Second", "2"),
                        TableColumnRequirement::Constant,
                    ),
                ],
                vec![vec!["3", "4"], vec!["5", "4"]],
            )
            .into(),
            json!({
                "type": "table", "description": "Table",
                "columns": [
                    {"key": "z", "description": "First", "default": "1", "constraint": {"type": "none"}, "requirement": "increasing"},
                    {"key": "a", "description": "Second", "default": "2", "constraint": {"type": "none"}, "requirement": "constant"}
                ],
                "default": [["3", "4"], ["5", "4"]]
            }),
        ),
        (
            "p_table_with_units",
            TableWithUnitsDefinition::new_with_column_requirements_and_default(
                "Units table",
                vec![
                    (
                        key("z"),
                        NumberWithUnitsDefinition::new_with_default(
                            "Distance",
                            "1 m",
                            UnitId::Length_Meter,
                        ),
                        TableColumnRequirement::ConstantOrDecreasing,
                    ),
                    (
                        key("a"),
                        NumberWithUnitsDefinition::new_with_default(
                            "Time",
                            "2 s",
                            UnitId::Time_Second,
                        ),
                        TableColumnRequirement::None,
                    ),
                ],
                vec![vec!["3 m", "4 s"], vec!["2 m", "5 s"]],
            )
            .into(),
            json!({
                "type": "table_with_units", "description": "Units table",
                "columns": [
                    {"key": "z", "description": "Distance", "default": "1 m", "constraint": {"type": "none"}, "requirement": "constant_or_decreasing", "preferred_unit": "length_meter"},
                    {"key": "a", "description": "Time", "default": "2 s", "constraint": {"type": "none"}, "requirement": "none", "preferred_unit": "time_second"}
                ],
                "default": [["3 m", "4 s"], ["2 m", "5 s"]]
            }),
        ),
        (
            "o_unit",
            UnitDefinition::new_with_default("Units", UnitFamilyId::Length, "u_length_meter")
                .into(),
            json!({"type": "unit", "description": "Units", "default": "u_length_meter", "unit_family": "length"}),
        ),
        (
            "n_tab",
            TabDefinition::new("Tab").into(),
            json!({"type": "tab", "description": "Tab"}),
        ),
        (
            "m_separator",
            SeparatorDefinition::new("").into(),
            json!({"type": "separator", "description": ""}),
        ),
    ]
}

/// Constructs a valid version-1 envelope with explicit empty definition arrays.
fn document(root: &str) -> Value {
    serde_json::from_str(&format!(
        "{{\"file-version\":\"1\",\"{root}\":{}}}",
        common::contents(root)
    ))
    .unwrap()
}

/// Reads and writes through the public file API and compares the entire file data.
fn round_trip(root: &str, path: &Utf8Path) {
    if root == "model" {
        let model = read_model_file(path).unwrap();
        write_model_file(model.clone(), path).unwrap();
        assert_eq!(read_model_file(path).unwrap(), model);
    } else {
        let component = read_component_file(path, None).unwrap();
        write_component_file(component.clone(), path).unwrap();
        common::assert_component_content_eq_ref(
            &component,
            &read_component_file(path, None).unwrap(),
        );
    }
}

/// Compares JSON fixtures with independent datastore expectations and the complete rewritten file.
fn assert_definitions_round_trip(path: &Utf8Path, definitions: &[(ItemDefinitionType, Value)]) {
    for root in ["model", "component"] {
        let mut document = document(root);
        let mut parameters = ParameterObjectDefinition::builder();
        let mut variables = VariableObjectDefinition::builder();
        let mut parameter_json = Vec::new();
        let mut variable_json = Vec::new();
        for (index, (definition, value)) in definitions.iter().enumerate() {
            let parameter = ParameterKey::new(format!("p_item{index}").into()).unwrap();
            let variable = VariableKey::new(format!("v_item{index}").into()).unwrap();
            let mut parameter_value = value.clone();
            parameter_value["key"] = json!(parameter.as_str());
            let mut variable_value = value.clone();
            variable_value["key"] = json!(variable.as_str());
            parameter_json.push(parameter_value);
            variable_json.push(variable_value);
            parameters.insert(parameter, definition.clone());
            variables.insert(variable, definition.clone());
        }
        let parameters = parameters.finish();
        let variables = variables.finish();
        document[root]["parameter_definition"] = json!(parameter_json);
        document[root]["variable_definition"] = json!(variable_json);
        fs::write(path, serde_json::to_string(&document).unwrap()).unwrap();

        if root == "model" {
            let model = read_model_file(path).unwrap();
            assert_eq!(model.parameter_definition(), &parameters);
            assert_eq!(model.variable_definition(), &variables);
        } else {
            let component = read_component_file(path, None).unwrap();
            assert_eq!(component.parameter_definition(), &parameters);
            assert_eq!(component.variable_definition(), &variables);
        }
        round_trip(root, path);
        let written: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        assert_eq!(written, document);
    }
}

#[test]
fn all_definition_types_round_trip_in_file_order() {
    let directory = TestDirectory::new();
    let path = directory.path.join("definitions.json");
    for root in ["model", "component"] {
        let mut document = document(root);
        let mut parameters = ParameterObjectDefinition::builder();
        let mut variables = VariableObjectDefinition::builder();
        let mut parameter_json = Vec::new();
        let mut variable_json = Vec::new();
        for (name, definition, value) in item_cases() {
            let parameter = ParameterKey::new(format!("p_{name}").into()).unwrap();
            let variable = VariableKey::new(format!("v_{name}").into()).unwrap();
            let mut parameter_value = value.clone();
            parameter_value["key"] = json!(parameter.as_str());
            let mut variable_value = value;
            variable_value["key"] = json!(variable.as_str());
            parameter_json.push(parameter_value);
            variable_json.push(variable_value);
            parameters.insert(parameter, definition.clone());
            variables.insert(variable, definition);
        }
        let parameters = parameters.finish();
        let variables = variables.finish();
        document[root]["parameter_definition"] = json!(parameter_json);
        document[root]["variable_definition"] = json!(variable_json);
        fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();

        if root == "model" {
            let model = read_model_file(&path).unwrap();
            assert_eq!(model.parameter_definition(), &parameters);
            assert_eq!(model.variable_definition(), &variables);
        } else {
            let component = read_component_file(&path, None).unwrap();
            assert_eq!(component.parameter_definition(), &parameters);
            assert_eq!(component.variable_definition(), &variables);
        }
        round_trip(root, &path);
        let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(written, document);
    }
}

#[test]
fn numeric_constraint_bounds_round_trip_exactly() {
    let directory = TestDirectory::new();
    let path = directory.path.join("numeric-bounds.json");
    let bound = 1.693_099_406_082_845_5;
    let definitions = [
        (
            NumberDefinition::new_with_constraint("Minimum", NumberConstraint::min(bound, true))
                .into(),
            json!({
                "type": "number", "description": "Minimum", "default": "",
                "constraint": {"type": "min", "min": bound, "inclusive": true}
            }),
        ),
        (
            NumberDefinition::new_with_constraint("Maximum", NumberConstraint::max(bound, false))
                .into(),
            json!({
                "type": "number", "description": "Maximum", "default": "",
                "constraint": {"type": "max", "max": bound, "inclusive": false}
            }),
        ),
        (
            NumberWithUnitsDefinition::new_with_constraint(
                "Range",
                NumberConstraint::range(-bound, bound, false, true),
                UnitId::Length_Meter,
            )
            .into(),
            json!({
                "type": "number_with_units", "description": "Range", "default": "",
                "constraint": {
                    "type": "range", "min": -bound, "max": bound,
                    "min_inclusive": false, "max_inclusive": true
                },
                "preferred_unit": "length_meter"
            }),
        ),
    ];
    assert_definitions_round_trip(&path, &definitions);
}

#[test]
fn normalized_definitions_round_trip() {
    let directory = TestDirectory::new();
    let path = directory.path.join("normalized-definitions.json");
    let choice = ChoiceDefinition::new_with_default(
        "Choice",
        vec![
            ChoiceItemDefinition::new(key("same"), "Old"),
            ChoiceItemDefinition::new(key("first"), "First"),
            ChoiceItemDefinition::new(key("same"), "Last"),
        ],
        "same",
    );
    let map = MapDefinition::new_with_default(
        "Map",
        vec![
            (
                key("label"),
                MapItemDefinition::from(StringDefinition::new_with_default("Label", "base")),
            ),
            (
                key("rows"),
                TableDefinition::new_with_default(
                    "Rows",
                    vec![(key("value"), NumberDefinition::new("Value"))],
                    vec![vec!["42"]],
                )
                .into(),
            ),
            (
                key("units"),
                TableWithUnitsDefinition::new(
                    "Units",
                    vec![(
                        key("length"),
                        NumberWithUnitsDefinition::new("Length", UnitId::Length_Meter),
                    )],
                )
                .into(),
            ),
            (key("choice"), choice.clone().into()),
        ],
        BTreeMap::from([
            (
                key("entry"),
                vec![
                    MapItemDefault::table(vec![vec!["wrong kind"]]),
                    MapItemDefault::scalar("wrong kind"),
                    MapItemDefault::scalar("wrong kind"),
                    MapItemDefault::scalar("same"),
                    MapItemDefault::scalar("extra"),
                ],
            ),
            (key("omitted"), Vec::new()),
            (key("partial"), vec![MapItemDefault::scalar("custom")]),
        ]),
    );
    let choice_json = json!({
        "type": "choice", "description": "Choice", "default": "same",
        "items": [
            {"id": "first", "description": "First"},
            {"id": "same", "description": "Last"}
        ]
    });
    let mut nested_choice = choice_json.clone();
    nested_choice["key"] = json!("choice");
    let map_json = json!({
        "type": "map", "description": "Map",
        "items": [
            {"key": "label", "type": "string", "description": "Label", "default": "base"},
            {
                "key": "rows", "type": "table", "description": "Rows",
                "columns": [{
                    "key": "value", "description": "Value", "default": "",
                    "constraint": {"type": "none"}, "requirement": "none"
                }],
                "default": [["42"]]
            },
            {
                "key": "units", "type": "table_with_units", "description": "Units",
                "columns": [{
                    "key": "length", "description": "Length", "default": "",
                    "constraint": {"type": "none"}, "requirement": "none",
                    "preferred_unit": "length_meter"
                }],
                "default": null
            },
            nested_choice
        ],
        "default": {
            "entry": ["base", [["42"]], [], "same"],
            "omitted": [],
            "partial": ["custom"]
        }
    });
    assert_definitions_round_trip(
        &path,
        &[(choice.into(), choice_json), (map.into(), map_json)],
    );
}

#[test]
fn readers_normalize_choice_ids_and_map_defaults() {
    let directory = TestDirectory::new();
    let path = directory.path.join("reader-normalization.json");
    let (_, _, mut choice) = item_cases()
        .into_iter()
        .find(|(name, _, _)| *name == "y_choice")
        .unwrap();
    let normalized_choice = choice.clone();
    choice["items"]
        .as_array_mut()
        .unwrap()
        .insert(0, json!({"id": "a", "description": "Old"}));
    let (_, _, mut map) = item_cases()
        .into_iter()
        .find(|(name, _, _)| *name == "u_map")
        .unwrap();
    let mut nested_choice = choice.clone();
    nested_choice["key"] = json!("mode");
    map["items"].as_array_mut().unwrap().push(nested_choice);
    map["default"] = json!({
        "entry": [[["wrong kind"]], "wrong kind", "wrong kind", [["wrong kind"]], "extra"],
        "rows": ["custom", [["1", "extra"], []], [[], ["2 m", "extra"]], "a"],
        "omitted": []
    });
    let mut normalized_map = map.clone();
    normalized_map["items"][3]["items"] = normalized_choice["items"].clone();
    normalized_map["default"] = json!({
        "entry": ["base", [], [], "z"],
        "rows": ["custom", [["1"], [""]], [[""], ["2 m"]], "a"],
        "omitted": []
    });

    for root in ["model", "component"] {
        let mut document = document(root);
        let mut expected = document.clone();
        for (field, prefix) in [("parameter_definition", "p"), ("variable_definition", "v")] {
            let mut choice = choice.clone();
            let mut map = map.clone();
            choice["key"] = json!(format!("{prefix}_choice"));
            map["key"] = json!(format!("{prefix}_map"));
            document[root][field] = json!([choice, map]);
            let mut normalized_choice = normalized_choice.clone();
            let mut normalized_map = normalized_map.clone();
            normalized_choice["key"] = json!(format!("{prefix}_choice"));
            normalized_map["key"] = json!(format!("{prefix}_map"));
            expected[root][field] = json!([normalized_choice, normalized_map]);
        }
        fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
        round_trip(root, &path);
        let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(written, expected);
    }
}

#[test]
fn collection_defaults_remain_explicitly_null_or_empty() {
    let directory = TestDirectory::new();
    let path = directory.path.join("defaults.json");
    for root in ["model", "component"] {
        for (kind, collection, empty) in [
            ("table", "columns", json!([])),
            ("table_with_units", "columns", json!([])),
            ("map", "items", json!({})),
        ] {
            for default in [Value::Null, empty] {
                let mut document = document(root);
                for (field, prefix) in [("parameter_definition", "p"), ("variable_definition", "v")]
                {
                    let mut item = json!({
                        "key": format!("{prefix}_collection"), "type": kind, "description": "",
                        "default": default
                    });
                    item[collection] = json!([]);
                    document[root][field] = json!([item]);
                }
                fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
                round_trip(root, &path);
                let written: Value =
                    serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
                assert_eq!(written, document);
            }
        }
    }
}
