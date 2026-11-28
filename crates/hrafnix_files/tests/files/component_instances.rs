//! Saved component instances in model and component documents.

use super::common::{self, TestDirectory};

use camino::Utf8Path;
use hrafnix_datastore::definition::{NumberDefinition, ParameterObjectDefinition, UnitDefinition};
use hrafnix_datastore::frozen::{ItemFrozen, NumberFrozen, ParameterObjectFrozen, UnitFrozen};
use hrafnix_files::data::component_instance::{
    ComponentInstance, ComponentSource, PackageReference, PackageVersion,
};
use hrafnix_files::data::graphics::Rotation;
use hrafnix_files::formats::component::{read_component_file, write_component_file};
use hrafnix_files::formats::model::{read_model_file, write_model_file};
use hrafnix_keys::parameter_key;
use hrafnix_message::message::Message;
use hrafnix_units::UnitFamilyId;
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs};

/// Example file-backed sensor and heater followed by a built-in clock.
fn instances_json() -> Value {
    json!([
        {
            "id": "temperature_sensor",
            "source": {
                "type": "file",
                "package": {
                    "name": "hrafnix.sensors",
                    "version": {"major": 1, "minor": 2, "patch": 3}
                },
                "path": "temperature_sensor.hxc"
            },
            "position": {"x": 150, "y": 0},
            "gui_metadata": {
                "dimensions": {"width": 120, "height": 60},
                "rotation": 0,
                "icon": "temperature-sensor.svg",
                "ports": [{
                    "id": "temperature",
                    "type": "output",
                    "location": {"x": 120, "y": 30}
                }]
            },
            "parameters": [{
                "key": "p_unit",
                "type": "unit",
                "definition": {
                    "description": "Unit",
                    "unit_family": "temperature",
                    "default": "u_temperature_celsius"
                },
                "value": "u_temperature_celsius"
            }]
        },
        {
            "id": "heater",
            "source": {
                "type": "file",
                "path": "heater.hxc"
            },
            "position": {"x": 320, "y": 0},
            "gui_metadata": {
                "dimensions": {"width": 120, "height": 60},
                "rotation": 90,
                "icon": "heater.svg",
                "ports": [{
                    "id": "temperature_input",
                    "type": "input",
                    "location": {"x": 0, "y": 30}
                }]
            },
            "parameters": [{
                "key": "p_maximum_power",
                "type": "number",
                "definition": {
                    "description": "Maximum power",
                    "default": "2000",
                    "constraint": {"type": "none"}
                },
                "value": "2000"
            }]
        },
        {
            "id": "simulation_clock",
            "source": {"type": "builtin", "name": "builtin.clock", "version": 1},
            "position": {"x": 0, "y": 0},
            "gui_metadata": {
                "dimensions": {"width": 100, "height": 50},
                "rotation": 180,
                "icon": "",
                "ports": [{
                    "id": "time",
                    "type": "output",
                    "location": {"x": 100, "y": 25}
                }]
            },
            "parameters": []
        }
    ])
}

/// Creates an otherwise minimal model or component document.
fn document(root: &str, instances: Value) -> Value {
    let mut document: Value = serde_json::from_str(&format!(
        "{{\"file-version\":\"1\",\"{root}\":{}}}",
        common::contents(root)
    ))
    .unwrap();
    document[root]["components"] = instances;
    document
}

/// Reads saved instances through either public file API.
fn read(root: &str, path: &Utf8Path) -> Result<Vec<ComponentInstance>, Message> {
    if root == "model" {
        read_model_file(path).map(|file| file.components().to_vec())
    } else {
        read_component_file(path, None).map(|file| file.components().to_vec())
    }
}

/// Saves a seeded document unchanged through its public reader and writer.
fn write(root: &str, path: &Utf8Path) -> Result<(), Message> {
    if root == "model" {
        write_model_file(read_model_file(path)?, path)
    } else {
        write_component_file(read_component_file(path, None)?, path)
    }
}

/// Verifies JSON-schema failures retain the repository's localized diagnostics.
fn assert_rejected(root: &str, path: &Utf8Path, document: &str) {
    fs::write(path, document).unwrap();
    let error = read(root, path).unwrap_err();
    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_json_data",
        "{document}"
    );
    assert_eq!(
        error.translate_data().message_params()["path"].as_str(),
        path.as_str(),
        "{document}"
    );
    assert!(error.extra_detail().is_some(), "{document}");
}

/// Verifies post-parse validation failures report their own localized diagnostic.
fn assert_rejected_with(root: &str, path: &Utf8Path, document: &str, message_key: &str) {
    fs::write(path, document).unwrap();
    let error = read(root, path).unwrap_err();
    assert_eq!(
        error.translate_data().message_key().as_str(),
        message_key,
        "{document}"
    );
}

#[test]
fn example_instances_round_trip_with_sources_snapshots_and_order() {
    let directory = TestDirectory::new();
    let path = directory.path.join("instances.json");
    for root in ["model", "component"] {
        let document = document(root, instances_json());
        fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();

        let instances = read(root, &path).unwrap();
        assert_eq!(
            instances
                .iter()
                .map(|instance| instance.id().as_str())
                .collect::<Vec<_>>(),
            ["temperature_sensor", "heater", "simulation_clock"]
        );
        assert_eq!(
            instances[0].source(),
            &ComponentSource::File {
                package: Some(PackageReference::new(
                    "hrafnix.sensors".into(),
                    PackageVersion::new(1, 2, 3),
                )),
                path: "temperature_sensor.hxc".into(),
            }
        );
        assert_eq!(
            instances[1].source(),
            &ComponentSource::File {
                package: None,
                path: "heater.hxc".into(),
            }
        );
        assert_eq!(
            instances[2].source(),
            &ComponentSource::Builtin {
                name: "builtin.clock".into(),
                version: 1,
            }
        );
        assert_eq!(
            instances
                .iter()
                .map(|instance| (instance.position().x(), instance.position().y()))
                .collect::<Vec<_>>(),
            [(150, 0), (320, 0), (0, 0)]
        );
        assert_eq!(instances[0].gui_metadata().rotation(), Rotation::Degrees0);
        assert_eq!(instances[1].gui_metadata().rotation(), Rotation::Degrees90);
        assert_eq!(instances[2].gui_metadata().rotation(), Rotation::Degrees180);
        let unit = UnitDefinition::new_with_default(
            "Unit",
            UnitFamilyId::Temperature,
            "u_temperature_celsius",
        );
        assert_eq!(
            instances[0].parameters(),
            &ParameterObjectFrozen::new_with_items(
                ParameterObjectDefinition::builder()
                    .with(parameter_key!("p_unit"), unit.clone())
                    .finish(),
                BTreeMap::from([(
                    parameter_key!("p_unit").into(),
                    ItemFrozen::Unit(UnitFrozen::new(unit)),
                )]),
            )
            .unwrap()
        );
        let power = NumberDefinition::new_with_default("Maximum power", "2000");
        assert_eq!(
            instances[1].parameters(),
            &ParameterObjectFrozen::new_with_items(
                ParameterObjectDefinition::builder()
                    .with(parameter_key!("p_maximum_power"), power.clone())
                    .finish(),
                BTreeMap::from([(
                    parameter_key!("p_maximum_power").into(),
                    ItemFrozen::Number(NumberFrozen::new(power)),
                )]),
            )
            .unwrap()
        );
        assert_eq!(
            instances[2].parameters(),
            &ParameterObjectFrozen::new(ParameterObjectDefinition::default())
        );

        write(root, &path).unwrap();

        assert_eq!(read(root, &path).unwrap(), instances);
        let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(written, document);
        assert_eq!(directory.entries(), ["instances.json"]);
    }
}

#[test]
fn gui_rotations_round_trip_without_changing_icon_local_geometry() {
    let directory = TestDirectory::new();
    let path = directory.path.join("rotations.json");
    for root in ["model", "component"] {
        let mut document = document(root, instances_json());
        fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
        let original = read(root, &path).unwrap().remove(0);
        for (degrees, rotation) in [
            (0, Rotation::Degrees0),
            (90, Rotation::Degrees90),
            (225, Rotation::Degrees270),
        ] {
            document[root]["components"][0]["gui_metadata"]["rotation"] = json!(degrees);
            fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
            let instances = read(root, &path).unwrap();
            let instance = &instances[0];
            assert_eq!(instance.gui_metadata().rotation(), rotation);
            assert_eq!(instance.position(), original.position());
            assert_eq!(
                instance.gui_metadata().dimensions(),
                original.gui_metadata().dimensions(),
            );
            assert_eq!(
                instance.gui_metadata().icon(),
                original.gui_metadata().icon()
            );
            assert_eq!(
                instance.gui_metadata().ports(),
                original.gui_metadata().ports()
            );
            assert_eq!(
                instance.interface_hash() == original.interface_hash(),
                rotation == original.gui_metadata().rotation(),
            );
            assert_eq!(
                instance == &original,
                rotation == original.gui_metadata().rotation(),
            );

            write(root, &path).unwrap();

            assert_eq!(read(root, &path).unwrap(), instances);
            let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
            document[root]["components"][0]["gui_metadata"]["rotation"] = json!(rotation.degrees());
            assert_eq!(written, document);
        }
    }
}

#[test]
fn null_packages_are_written_without_package_or_version_fields() {
    let directory = TestDirectory::new();
    let path = directory.path.join("loose-files.json");
    for root in ["model", "component"] {
        let document = document(root, instances_json());
        let mut value = document.clone();
        value[root]["components"][1]["source"]["package"] = Value::Null;
        fs::write(&path, serde_json::to_string(&value).unwrap()).unwrap();
        let instances = read(root, &path).unwrap();
        assert!(matches!(
            instances[1].source(),
            ComponentSource::File { package: None, .. }
        ));

        write(root, &path).unwrap();

        assert_eq!(read(root, &path).unwrap(), instances);
        let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(written, document);
    }
}

#[test]
fn instance_conversion_errors_propagate_from_both_readers() {
    let directory = TestDirectory::new();
    let path = directory.path.join("invalid-fields.json");
    for root in ["model", "component"] {
        let document = document(root, instances_json());
        let mut invalid_id = document.clone();
        invalid_id[root]["components"][0]["id"] = json!("");
        assert_rejected_with(
            root,
            &path,
            &serde_json::to_string(&invalid_id).unwrap(),
            "datastore_key_empty",
        );
        let mut duplicate_id = document.clone();
        duplicate_id[root]["components"][2]["id"] = json!("temperature_sensor");
        assert_rejected_with(
            root,
            &path,
            &serde_json::to_string(&duplicate_id).unwrap(),
            "files_duplicate_component_id",
        );
        let mut duplicate_parameter = document.clone();
        duplicate_parameter[root]["components"][0]["parameters"] = json!([
            document[root]["components"][0]["parameters"][0],
            document[root]["components"][0]["parameters"][0]
        ]);
        assert_rejected_with(
            root,
            &path,
            &serde_json::to_string(&duplicate_parameter).unwrap(),
            "files_duplicate_parameter_key",
        );
    }
}

#[test]
fn duplicate_fields_are_rejected_without_losing_raw_json_entries() {
    let directory = TestDirectory::new();
    let path = directory.path.join("duplicates.json");
    for root in ["model", "component"] {
        let value = document(root, instances_json());
        let document = serde_json::to_string(&value).unwrap();
        for (pointer, field) in [
            (format!("/{root}"), "components"),
            (format!("/{root}/components/0"), "id"),
        ] {
            let object = value.pointer(&pointer).unwrap();
            let original = serde_json::to_string(object).unwrap();
            let duplicate = format!(
                "{},\"{field}\":{}}}",
                original.strip_suffix('}').unwrap(),
                object[field]
            );
            let invalid = document.replacen(&original, &duplicate, 1);
            assert_ne!(invalid, document);
            assert_rejected(root, &path, &invalid);
        }
    }
}

#[test]
fn embedded_map_parameter_schema_errors_propagate_from_both_readers() {
    let directory = TestDirectory::new();
    let path = directory.path.join("map-parameters.json");
    for root in ["model", "component"] {
        let mut document = document(root, instances_json());
        document[root]["components"][0]["parameters"] = json!([{
            "key": "p_labels",
            "type": "map",
            "definition": {
                "description": "Labels",
                "items": [{"key": "name", "type": "string", "description": "", "default": ""}],
                "default": null
            },
            "value": {"first": ["sample"]}
        }]);
        fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
        let instances = read(root, &path).unwrap();
        write(root, &path).unwrap();
        assert_eq!(read(root, &path).unwrap(), instances);
        let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(written, document);
        for invalid in [json!({"first": []}), json!({"first": ["sample", "extra"]})] {
            document[root]["components"][0]["parameters"][0]["value"] = invalid;
            fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
            let error = read(root, &path).unwrap_err();
            assert_eq!(
                error.translate_data().message_key().as_str(),
                "datastore_schema_mismatch"
            );
        }
    }
}

#[test]
fn snapshot_expressions_parameter_order_and_unresolved_provenance_are_preserved() {
    let directory = TestDirectory::new();
    let path = directory.path.join("snapshot.json");
    for root in ["model", "component"] {
        let mut document = document(root, instances_json());
        document[root]["components"][0]["source"] = json!({
            "type": "file",
            "package": {"name": "", "version": {"major": u16::MAX, "minor": 0, "patch": 1}},
            "path": "../missing/component.hxc"
        });
        document[root]["components"][2]["source"] = json!({
            "type": "builtin", "name": "unknown.builtin", "version": u32::MAX
        });
        document[root]["components"][0]["position"] = json!({"x": i32::MIN, "y": i32::MAX});
        document[root]["components"][0]["parameters"] = json!([
            {
                "key": "p_z",
                "type": "string",
                "definition": {"description": "Z", "default": "\"default\""},
                "value": "\"unevaluated\" + p_a"
            },
            {
                "key": "p_a",
                "type": "number",
                "definition": {"description": "A", "default": "42", "constraint": {"type": "none"}},
                "value": "10 * 2"
            }
        ]);
        fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
        let instances = read(root, &path).unwrap();
        assert_eq!(
            (instances[0].position().x(), instances[0].position().y()),
            (i32::MIN, i32::MAX)
        );
        assert_eq!(
            instances[0]
                .parameters()
                .ordered_iter()
                .map(|(key, _)| key.as_str())
                .collect::<Vec<_>>(),
            ["p_z", "p_a"]
        );

        write(root, &path).unwrap();

        assert_eq!(read(root, &path).unwrap(), instances);
        let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(written, document);
        assert_eq!(directory.entries(), ["snapshot.json"]);
    }
}
