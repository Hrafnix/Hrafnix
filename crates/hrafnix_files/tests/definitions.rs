//! Document-level coverage of parameter and variable definition codecs.

#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::unwrap_used)]

#[allow(
    dead_code,
    reason = "Only document construction and temporary directories are needed here"
)]
mod common;

use camino::Utf8Path;
use common::TestDirectory;
use hrafnix_datastore::definition::{
    BooleanDefinition, ChoiceDefinition, ChoiceItemDefinition, FileDefinition, FolderDefinition,
    IntegerConstraint, IntegerDefinition, ItemDefinitionType, MapDefinition, MapItemDefault,
    MapItemDefinition, NumberConstraint, NumberDefinition, NumberWithUnitsDefinition,
    ParameterObjectDefinition, SeparatorDefinition, StringDefinition, TabDefinition,
    TableColumnRequirement, TableDefinition, TableWithUnitsDefinition, UnitDefinition,
    VariableObjectDefinition,
};
use hrafnix_files::{
    component::{read_component_file, write_component_file},
    model::{read_model_file, write_model_file},
};
use hrafnix_keys::{ParameterKey, StoreKey, VariableKey};
use hrafnix_message::message::Message;
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
        let component = read_component_file(path).unwrap();
        write_component_file(component.clone(), path).unwrap();
        assert_eq!(read_component_file(path).unwrap(), component);
    }
}

/// Saves constructed datastore definitions and compares reloaded model and component data.
fn assert_definitions_round_trip(path: &Utf8Path, definitions: &[ItemDefinitionType]) {
    for root in ["model", "component"] {
        let mut parameters = ParameterObjectDefinition::builder();
        let mut variables = VariableObjectDefinition::builder();
        for (index, definition) in definitions.iter().enumerate() {
            parameters.insert(
                ParameterKey::new(format!("p_item{index}").into()).unwrap(),
                definition.clone(),
            );
            variables.insert(
                VariableKey::new(format!("v_item{index}").into()).unwrap(),
                definition.clone(),
            );
        }
        fs::write(path, serde_json::to_string(&document(root)).unwrap()).unwrap();

        if root == "model" {
            let mut model = read_model_file(path).unwrap();
            model.parameter_definition = parameters.finish();
            model.variable_definition = variables.finish();
            write_model_file(model.clone(), path).unwrap();
            assert_eq!(read_model_file(path).unwrap(), model);
        } else {
            let mut component = read_component_file(path).unwrap();
            component.parameter_definition = parameters.finish();
            component.variable_definition = variables.finish();
            write_component_file(component.clone(), path).unwrap();
            assert_eq!(read_component_file(path).unwrap(), component);
        }
    }
}

/// Reads either document through the public API for rejection tests.
fn read(root: &str, path: &Utf8Path) -> Result<(), Message> {
    if root == "model" {
        read_model_file(path).map(|_| ())
    } else {
        read_component_file(path).map(|_| ())
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
            assert_eq!(model.parameter_definition, parameters);
            assert_eq!(model.variable_definition, variables);
        } else {
            let component = read_component_file(&path).unwrap();
            assert_eq!(component.parameter_definition, parameters);
            assert_eq!(component.variable_definition, variables);
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
    let definitions: [ItemDefinitionType; 3] = [
        NumberDefinition::new_with_constraint("Minimum", NumberConstraint::min(bound, true)).into(),
        NumberDefinition::new_with_constraint("Maximum", NumberConstraint::max(bound, false))
            .into(),
        NumberWithUnitsDefinition::new_with_constraint(
            "Range",
            NumberConstraint::range(-bound, bound, false, true),
            UnitId::Length_Meter,
        )
        .into(),
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
    assert_definitions_round_trip(&path, &[choice.into(), map.into()]);
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

#[test]
fn definition_arrays_validate_keys_shapes_and_all_item_fields() {
    let directory = TestDirectory::new();
    let path = directory.path.join("invalid-definitions.json");
    for root in ["model", "component"] {
        for (field, prefix, other_prefix) in [
            ("parameter_definition", "p", "v"),
            ("variable_definition", "v", "p"),
        ] {
            let base = document(root);
            let mut missing = base.clone();
            missing[root].as_object_mut().unwrap().remove(field);
            let mut invalid_documents = vec![missing];
            let mut invalid_key_documents = Vec::new();
            for value in [
                Value::Null,
                json!({}),
                json!(false),
                json!(1),
                json!("items"),
                json!([null]),
                json!([[]]),
            ] {
                let mut document = base.clone();
                document[root][field] = value;
                invalid_documents.push(document);
            }
            for (index, (_, _, mut item)) in item_cases().into_iter().enumerate() {
                item["key"] = json!(format!("{prefix}_item"));
                let mut valid = base.clone();
                valid[root][field] = json!([item]);
                fs::write(&path, serde_json::to_string(&valid).unwrap()).unwrap();
                read(root, &path).unwrap();

                for name in item.as_object().unwrap().keys() {
                    let mut document = valid.clone();
                    document[root][field][0]
                        .as_object_mut()
                        .unwrap()
                        .remove(name);
                    invalid_documents.push(document);
                }
                let mut unknown = valid.clone();
                unknown[root][field][0]["unexpected"] = json!(true);
                invalid_documents.push(unknown);
                for invalid_key in [json!(1), json!({"key": format!("{prefix}_item")})] {
                    let mut document = valid.clone();
                    document[root][field][0]["key"] = invalid_key;
                    invalid_documents.push(document);
                }
                for (invalid_key, message_key) in [
                    (String::new(), "datastore_key_empty"),
                    ("plain".to_owned(), "datastore_key_invalid_prefix"),
                    (
                        format!("{other_prefix}_item"),
                        "datastore_key_invalid_prefix",
                    ),
                    (
                        format!("{prefix}_invalid key"),
                        "datastore_key_invalid_character",
                    ),
                ] {
                    let mut document = valid.clone();
                    document[root][field][0]["key"] = json!(invalid_key);
                    invalid_key_documents.push((document, message_key));
                }
                for invalid_type in [json!("unknown"), json!({"string": null}), json!(index)] {
                    let mut document = valid.clone();
                    document[root][field][0]["type"] = invalid_type;
                    invalid_documents.push(document);
                }
                if item["default"].is_string() {
                    for invalid_default in [Value::Null, json!(true), json!(2.5), json!([])] {
                        let mut document = valid.clone();
                        document[root][field][0]["default"] = invalid_default;
                        invalid_documents.push(document);
                    }
                }
            }
            for document in invalid_documents {
                fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
                let error = read(root, &path).expect_err(&document.to_string());
                assert_eq!(
                    error.translate_data().message_key().as_str(),
                    "files_json_data"
                );
                assert!(error.extra_detail().is_some());
            }
            for (document, message_key) in invalid_key_documents {
                fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
                let error = read(root, &path).expect_err(&document.to_string());
                assert_eq!(
                    error.translate_data().message_key().as_str(),
                    message_key,
                    "{document}"
                );
            }
        }
    }
}

#[test]
fn nested_item_and_constraint_tags_reject_numeric_variant_indexes() {
    let directory = TestDirectory::new();
    let path = directory.path.join("numeric-tags.json");
    for root in ["model", "component"] {
        for (field, prefix) in [("parameter_definition", "p"), ("variable_definition", "v")] {
            for (name, pointer, index) in [
                ("v_integer", "/constraint/type", 3),
                ("t_number", "/constraint/type", 1),
                ("s_number_with_units", "/constraint/type", 2),
                ("u_map", "/items/0/type", 6),
                ("u_map", "/items/1/type", 7),
                ("u_map", "/items/2/type", 8),
                ("u_map", "/items/1/columns/0/constraint/type", 0),
                ("u_map", "/items/2/columns/0/constraint/type", 0),
                ("q_table", "/columns/0/constraint/type", 0),
                ("p_table_with_units", "/columns/0/constraint/type", 0),
            ] {
                let (_, _, mut item) = item_cases()
                    .into_iter()
                    .find(|(candidate, _, _)| *candidate == name)
                    .unwrap();
                item["key"] = json!(format!("{prefix}_item"));
                *item.pointer_mut(pointer).unwrap() = json!(index);
                let mut document = document(root);
                document[root][field] = json!([item]);
                fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
                let error = read(root, &path).expect_err(&document.to_string());
                assert_eq!(
                    error.translate_data().message_key().as_str(),
                    "files_json_data"
                );
            }
        }
    }
}

#[test]
fn duplicate_definition_keys_and_json_fields_are_rejected() {
    let directory = TestDirectory::new();
    let path = directory.path.join("duplicates.json");
    for root in ["model", "component"] {
        for (field, prefix) in [("parameter_definition", "p"), ("variable_definition", "v")] {
            let item = format!(
                r#"{{"key":"{prefix}_item","type":"string","description":"","default":""}}"#
            );
            let duplicate_key_message = if prefix == "p" {
                "files_duplicate_parameter_key"
            } else {
                "files_duplicate_variable_key"
            };
            let mut invalid_arrays = vec![(format!("[{item},{item}]"), duplicate_key_message)];
            for (name, value) in [
                ("key", format!("\"{prefix}_item\"")),
                ("type", "\"string\"".to_owned()),
                ("description", "\"\"".to_owned()),
                ("default", "\"\"".to_owned()),
            ] {
                let duplicate = item.replace(
                    &format!("\"{name}\":{value}"),
                    &format!("\"{name}\":{value},\"{name}\":{value}"),
                );
                assert_ne!(duplicate, item);
                invalid_arrays.push((format!("[{duplicate}]"), "files_json_data"));
            }
            let contents = common::contents(root);
            let mut invalid_bodies: Vec<_> = invalid_arrays
                .iter()
                .map(|(array, message_key)| {
                    (
                        contents.replace(
                            &format!("\"{field}\": []"),
                            &format!("\"{field}\": {array}"),
                        ),
                        *message_key,
                    )
                })
                .collect();
            invalid_bodies.push((
                contents.replace(
                    &format!("\"{field}\": []"),
                    &format!("\"{field}\": [], \"{field}\": []"),
                ),
                "files_json_data",
            ));
            for (body, message_key) in invalid_bodies {
                let document = format!("{{\"file-version\":\"1\",\"{root}\":{body}}}");
                fs::write(&path, &document).unwrap();
                let error = read(root, &path).expect_err(&document);
                assert_eq!(
                    error.translate_data().message_key().as_str(),
                    message_key,
                    "{document}"
                );
            }
        }
    }
}
