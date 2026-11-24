//! Semantic file hashes through the public model and component readers.

#![allow(clippy::indexing_slicing, clippy::unwrap_used)]

use super::common::{self, TestDirectory};

use crate::component::{read_component_file, write_component_file};
use crate::model::{read_model_file, write_model_file};
use camino::Utf8Path;
use serde_json::{Value, json};
use std::fs;

/// Reads a semantic file hash and, for components, the interface hash.
fn read_hashes(root: &str, path: &Utf8Path) -> ([u8; 32], Option<[u8; 32]>) {
    if root == "model" {
        (read_model_file(path).unwrap().file_hash(), None)
    } else {
        let component = read_component_file(path, None).unwrap();
        (component.file_hash(), Some(component.interface_hash()))
    }
}

/// Builds a document exercising each persisted section.
fn document(root: &str) -> Value {
    let mut document: Value = serde_json::from_str(&format!(
        "{{\"file-version\":\"1\",\"{root}\":{}}}",
        common::contents(root)
    ))
    .unwrap();
    let data = &mut document[root];
    data["foreground_decorations"] = json!([
        {"id": "text", "position": {"x": 1, "y": 2},
         "dimensions": {"width": 30, "height": 40},
         "kind": {"Text": {"text": "Title", "font_size": 2000}}},
        {"id": "image", "position": {"x": 3, "y": 4},
         "dimensions": {"width": 50, "height": 60},
         "kind": {"Image": {"path": "image.png", "fit": "Contain"}}}
    ]);
    data["background_decorations"] = json!([
        {"id": "frame", "position": {"x": 5, "y": 6},
         "dimensions": {"width": 70, "height": 80}, "kind": "Frame"}
    ]);
    data["components"] = json!([
        {"id": "first", "source": {"type": "builtin", "name": "clock", "version": "1"},
         "position": {"x": 7, "y": 8},
         "gui_metadata": {"dimensions": {"width": 100, "height": 50}, "icon": "clock.svg",
             "ports": [{"id": "time", "type": "output", "location": {"x": 10, "y": 20}}]},
         "parameters": [
             {"key": "p_first", "type": "number",
              "definition": {"description": "First", "default": "1", "constraint": {"type": "none"}},
              "value": "2"},
             {"key": "p_second", "type": "number",
              "definition": {"description": "Second", "default": "3", "constraint": {"type": "none"}},
              "value": "4"}]},
        {"id": "second", "source": {"type": "file", "path": "sensor.hxc",
             "package": {"name": "sensors", "version": "1"}},
         "position": {"x": 9, "y": 10},
         "gui_metadata": {"dimensions": {"width": 100, "height": 50}, "icon": "", "ports": []},
         "parameters": []}
    ]);
    data["connections"] = json!([
        {"from_component": "first", "from_port": "time", "to_component": "second", "to_port": "input",
         "line": {"midpoints": [{"x": 11, "y": 12}, {"x": 13, "y": 14}]}}
    ]);
    data["translations"] = json!({"language": "en", "translations": {
        "Title": {"de": "Titel", "fr": "Titre"}, "Other": {"de": "Andere"}
    }});
    for (field, prefix) in [("parameter_definition", "p"), ("variable_definition", "v")] {
        data[field] = json!([
            {"key": format!("{prefix}_first"), "type": "number", "description": "First",
             "default": "1", "constraint": {"type": "none"}},
            {"key": format!("{prefix}_second"), "type": "number", "description": "Second",
             "default": "2", "constraint": {"type": "none"}}
        ]);
    }
    if root == "model" {
        data["global_settings"] = json!([
            {"key": "g_first", "type": "number",
             "definition": {"description": "First", "default": "1", "constraint": {"type": "none"}},
             "value": "2"},
            {"key": "g_second", "type": "number",
             "definition": {"description": "Second", "default": "3", "constraint": {"type": "none"}},
             "value": "4"}
        ]);
    }
    document
}

#[test]
fn formatting_and_round_trips_preserve_file_hashes() {
    for root in ["model", "component"] {
        let directory = TestDirectory::new();
        let path = directory.path.join("hash.json");
        let document = document(root);
        let compact = serde_json::to_string(&document).unwrap();
        fs::write(&path, &compact).unwrap();
        let expected = read_hashes(root, &path);
        let pretty = serde_json::to_string_pretty(&document).unwrap();
        for text in [
            pretty,
            format!("\u{feff}{compact}\r\n"),
            compact.replace("Title", "\\u0054itle"),
            format!(
                "{{\"{root}\":{},\"file-version\":\"1\"}}",
                serde_json::to_string(&document[root]).unwrap()
            ),
        ] {
            fs::write(&path, text).unwrap();
            assert_eq!(read_hashes(root, &path), expected);
        }
        if root == "model" {
            write_model_file(read_model_file(&path).unwrap(), &path).unwrap();
        } else {
            write_component_file(read_component_file(&path, None).unwrap(), &path).unwrap();
        }
        assert_eq!(read_hashes(root, &path), expected);
    }
}

#[test]
fn file_hashes_cover_data_beyond_the_interface() {
    for root in ["model", "component"] {
        let directory = TestDirectory::new();
        let path = directory.path.join("hash.json");
        let original = document(root);
        fs::write(&path, serde_json::to_string(&original).unwrap()).unwrap();
        let expected = read_hashes(root, &path);
        let mut mutations = vec![
            ("/foreground_decorations/0/id", json!("other")),
            ("/foreground_decorations/0/position/x", json!(-1)),
            ("/foreground_decorations/0/position/y", json!(-2)),
            ("/foreground_decorations/0/dimensions/width", json!(31)),
            ("/foreground_decorations/0/dimensions/height", json!(41)),
            ("/foreground_decorations/0/kind/Text/text", json!("Changed")),
            ("/foreground_decorations/0/kind/Text/font_size", json!(2100)),
            (
                "/foreground_decorations/1/kind/Image/path",
                json!("other.png"),
            ),
            ("/foreground_decorations/1/kind/Image/fit", json!("Cover")),
            ("/foreground_decorations/1/kind/Image/fit", json!("Fill")),
            ("/background_decorations/0/id", json!("other")),
            ("/background_decorations/0/position/x", json!(-5)),
            ("/background_decorations/0/position/y", json!(-6)),
            ("/background_decorations/0/dimensions/width", json!(71)),
            ("/background_decorations/0/dimensions/height", json!(81)),
            ("/components/0/id", json!("other")),
            ("/components/0/source/name", json!("other")),
            ("/components/0/source/version", json!("2")),
            ("/components/0/position/x", json!(-7)),
            ("/components/0/position/y", json!(-8)),
            ("/components/0/gui_metadata/icon", json!("other.svg")),
            ("/components/0/gui_metadata/dimensions/width", json!(101)),
            ("/components/0/gui_metadata/dimensions/height", json!(51)),
            ("/components/0/gui_metadata/ports/0/id", json!("other")),
            ("/components/0/gui_metadata/ports/0/type", json!("input")),
            ("/components/0/gui_metadata/ports/0/location/x", json!(11)),
            ("/components/0/gui_metadata/ports/0/location/y", json!(21)),
            ("/components/0/parameters/0/value", json!("5")),
            (
                "/components/0/parameters/0/definition/description",
                json!("Changed"),
            ),
            ("/components/0/parameters/0/definition/default", json!("5")),
            ("/components/1/source/path", json!("other.hxc")),
            ("/components/1/source/package/name", json!("other")),
            ("/components/1/source/package/version", json!("2")),
            ("/connections/0/from_component", json!("other")),
            ("/connections/0/from_port", json!("other")),
            ("/connections/0/to_component", json!("other")),
            ("/connections/0/to_port", json!("other")),
            ("/connections/0/line/midpoints/0/x", json!(-11)),
            ("/connections/0/line/midpoints/0/y", json!(-12)),
            ("/translations/language", json!("de")),
            ("/translations/translations/Title/de", json!("Changed")),
            ("/variable_definition/0/description", json!("Changed")),
            ("/variable_definition/0/default", json!("5")),
        ];
        if root == "model" {
            mutations.extend([
                ("/global_settings/0/value", json!("5")),
                (
                    "/global_settings/0/definition/description",
                    json!("Changed"),
                ),
                ("/global_settings/0/definition/default", json!("5")),
            ]);
        }
        for (pointer, value) in mutations {
            let mut changed = original.clone();
            *changed[root].pointer_mut(pointer).unwrap() = value;
            fs::write(&path, serde_json::to_string(&changed).unwrap()).unwrap();
            let actual = read_hashes(root, &path);
            assert_ne!(actual.0, expected.0, "{root}{pointer}");
            assert_eq!(actual.1, expected.1, "{root}{pointer}");
        }
        let mut arrays = vec![
            "/foreground_decorations",
            "/components",
            "/components/0/parameters",
            "/connections/0/line/midpoints",
            "/variable_definition",
        ];
        if root == "model" {
            arrays.push("/global_settings");
        }
        for pointer in arrays {
            let mut changed = original.clone();
            changed[root]
                .pointer_mut(pointer)
                .unwrap()
                .as_array_mut()
                .unwrap()
                .reverse();
            fs::write(&path, serde_json::to_string(&changed).unwrap()).unwrap();
            assert_ne!(read_hashes(root, &path).0, expected.0, "{root}{pointer}");
        }
    }
}

#[test]
fn file_hashes_cover_interface_fields() {
    for root in ["model", "component"] {
        let directory = TestDirectory::new();
        let path = directory.path.join("hash.json");
        let original = document(root);
        fs::write(&path, serde_json::to_string(&original).unwrap()).unwrap();
        let expected = read_hashes(root, &path);
        let mut mutations = vec![
            ("/metadata/display-name", json!("Changed")),
            ("/metadata/description", json!("Changed")),
            ("/metadata/author", json!("Changed")),
            ("/metadata/minimum-version-major", json!(1)),
            ("/metadata/minimum-version-minor", json!(1)),
            ("/parameter_definition/0/description", json!("Changed")),
            ("/parameter_definition/0/default", json!("5")),
        ];
        if root == "component" {
            mutations.extend([
                ("/gui_metadata/icon", json!("other.svg")),
                ("/gui_metadata/dimensions/width", json!(101)),
                ("/gui_metadata/dimensions/height", json!(51)),
                (
                    "/gui_metadata/ports",
                    json!([{"id": "time", "type": "output", "location": {"x": 10, "y": 20}}]),
                ),
            ]);
        }
        for (pointer, value) in mutations {
            let mut changed = original.clone();
            *changed[root].pointer_mut(pointer).unwrap() = value;
            fs::write(&path, serde_json::to_string(&changed).unwrap()).unwrap();
            let actual = read_hashes(root, &path);
            assert_ne!(actual.0, expected.0, "{root}{pointer}");
            if root == "component" {
                assert_ne!(actual.1, expected.1, "{root}{pointer}");
            }
        }
        let mut changed = original;
        changed[root]["parameter_definition"]
            .as_array_mut()
            .unwrap()
            .reverse();
        fs::write(&path, serde_json::to_string(&changed).unwrap()).unwrap();
        assert_ne!(read_hashes(root, &path).0, expected.0);
    }
}

#[test]
fn component_file_hash_includes_loaded_provenance() {
    let directory = TestDirectory::new();
    let first = directory.path.join("first.hxc");
    let second = directory.path.join("second.hxc");
    let text = serde_json::to_string(&document("component")).unwrap();
    fs::write(&first, &text).unwrap();
    fs::write(&second, &text).unwrap();
    assert_ne!(
        read_hashes("component", &first),
        read_hashes("component", &second)
    );
    let without_package = read_component_file(&first, None).unwrap();
    let with_package = read_component_file(
        &first,
        Some(crate::source::data::PackageReference::new(
            "package".into(),
            "1".into(),
        )),
    )
    .unwrap();
    assert_ne!(without_package.file_hash(), with_package.file_hash());
}

/// Builds frozen table, unit-table, and map items whose table references can be mutated.
fn frozen_table_items(prefix: &str) -> Value {
    json!([
        {
            "key": format!("{prefix}_table"),
            "type": "table",
            "definition": {
                "description": "Table",
                "columns": [
                    {"key": "value", "description": "", "default": "", "constraint": {"type": "none"}, "requirement": "none"}
                ],
                "default": null
            },
            "parameter": "",
            "rows": [["1"]]
        },
        {
            "key": format!("{prefix}_table_units"),
            "type": "table_with_units",
            "definition": {
                "description": "Units table",
                "columns": [
                    {"key": "distance", "description": "", "default": "", "constraint": {"type": "none"}, "requirement": "none", "preferred_unit": "length_meter"}
                ],
                "default": null
            },
            "parameter": "",
            "units": ["length_meter"],
            "rows": [["2"]]
        },
        {
            "key": format!("{prefix}_map"),
            "type": "map",
            "definition": {
                "description": "Map",
                "items": [
                    {
                        "key": "table",
                        "type": "table",
                        "description": "Rows",
                        "columns": [
                            {"key": "value", "description": "", "default": "", "constraint": {"type": "none"}, "requirement": "none"}
                        ],
                        "default": null
                    },
                    {
                        "key": "table_units",
                        "type": "table_with_units",
                        "description": "Distances",
                        "columns": [
                            {"key": "distance", "description": "", "default": "", "constraint": {"type": "none"}, "requirement": "none", "preferred_unit": "length_meter"}
                        ],
                        "default": null
                    }
                ],
                "default": null
            },
            "value": {
                "entry": [
                    {"parameter": "", "rows": [["3"]]},
                    {"parameter": "", "units": ["length_meter"], "rows": [["4"]]}
                ]
            }
        }
    ])
}

#[test]
fn file_hashes_cover_table_parameter_references() {
    for root in ["model", "component"] {
        let directory = TestDirectory::new();
        let path = directory.path.join("hash_table_refs.json");
        let mut original = document(root);
        original[root]["components"][0]["parameters"] = frozen_table_items("p");
        let mut mutations = vec![
            ("/components/0/parameters/0/parameter", json!("p_ref")),
            ("/components/0/parameters/1/parameter", json!("p_units_ref")),
            (
                "/components/0/parameters/2/value/entry/0/parameter",
                json!("p_map_ref"),
            ),
            (
                "/components/0/parameters/2/value/entry/1/parameter",
                json!("p_map_units_ref"),
            ),
        ];
        if root == "model" {
            original[root]["global_settings"] = frozen_table_items("g");
            mutations.extend([
                ("/global_settings/0/parameter", json!("g_ref")),
                ("/global_settings/1/parameter", json!("g_units_ref")),
                (
                    "/global_settings/2/value/entry/0/parameter",
                    json!("g_map_ref"),
                ),
                (
                    "/global_settings/2/value/entry/1/parameter",
                    json!("g_map_units_ref"),
                ),
            ]);
        }

        fs::write(&path, serde_json::to_string(&original).unwrap()).unwrap();
        let expected = read_hashes(root, &path);

        for (pointer, value) in mutations {
            let mut changed = original.clone();
            *changed[root].pointer_mut(pointer).unwrap() = value;
            fs::write(&path, serde_json::to_string(&changed).unwrap()).unwrap();
            let actual = read_hashes(root, &path);
            assert_ne!(actual.0, expected.0, "{root}{pointer}");
            assert_eq!(actual.1, expected.1, "{root}{pointer}");
        }
    }
}
