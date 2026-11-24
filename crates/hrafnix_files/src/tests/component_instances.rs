//! Saved component instances in model and component documents.

#![allow(clippy::indexing_slicing, clippy::unwrap_used)]

use super::common::{self, TestDirectory};

use crate::component::{data::ComponentFile, read_component_file, write_component_file};
use crate::component_instance::data::{ComponentInstance, ComponentSource, PackageReference};
use crate::graphics::data::{Position, Rotation};
use crate::metadata_gui::data::{GuiMetadata, Port, PortLocation};
use crate::model::{data::ModelFile, read_model_file, write_model_file};
use camino::Utf8Path;
use hrafnix_datastore::definition::{NumberDefinition, ParameterObjectDefinition, UnitDefinition};
use hrafnix_datastore::frozen::{ItemFrozen, NumberFrozen, ParameterObjectFrozen, UnitFrozen};
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
                "package": {"name": "hrafnix.sensors", "version": "1"},
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
            "source": {"type": "builtin", "name": "builtin.clock", "version": "1"},
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

/// Replaces the instances in an existing document through its public writer.
fn write(root: &str, instances: Vec<ComponentInstance>, path: &Utf8Path) -> Result<(), Message> {
    if root == "model" {
        let file = read_model_file(path)?;
        let updated = ModelFile::new(
            file.metadata().clone(),
            file.foreground_decorations().to_vec(),
            file.background_decorations().to_vec(),
            file.global_settings().clone(),
            instances,
            file.connections().to_vec(),
            file.translations().clone(),
            file.parameter_definition().clone(),
            file.variable_definition().clone(),
        );
        write_model_file(updated, path)
    } else {
        let file = read_component_file(path, None)?;
        let updated = ComponentFile::new(
            file.metadata().clone(),
            file.source().clone(),
            file.gui_metadata().clone(),
            file.foreground_decorations().to_vec(),
            file.background_decorations().to_vec(),
            instances,
            file.connections().to_vec(),
            file.translations().clone(),
            file.parameter_definition().clone(),
            file.variable_definition().clone(),
        );
        write_component_file(updated, path)
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
                package: Some(PackageReference::new("hrafnix.sensors".into(), "1".into())),
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
                version: "1".into(),
            }
        );
        assert_eq!(instances[0].position(), Position::new(150, 0));
        assert_eq!(instances[1].position(), Position::new(320, 0));
        assert_eq!(instances[2].position(), Position::new(0, 0));
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

        write(root, instances.clone(), &path).unwrap();

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
            (44, Rotation::Degrees0),
            (45, Rotation::Degrees90),
            (90, Rotation::Degrees90),
            (134, Rotation::Degrees90),
            (135, Rotation::Degrees180),
            (180, Rotation::Degrees180),
            (224, Rotation::Degrees180),
            (225, Rotation::Degrees270),
            (270, Rotation::Degrees270),
            (314, Rotation::Degrees270),
            (315, Rotation::Degrees0),
            (360, Rotation::Degrees0),
            (405, Rotation::Degrees90),
            (u16::MAX, Rotation::Degrees0),
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

            write(root, instances.clone(), &path).unwrap();

            assert_eq!(read(root, &path).unwrap(), instances);
            let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
            document[root]["components"][0]["gui_metadata"]["rotation"] = json!(rotation.degrees());
            assert_eq!(written, document);
        }
    }
}

#[test]
fn package_references_are_strict_objects_and_replace_file_source_versions() {
    let directory = TestDirectory::new();
    let path = directory.path.join("packages.json");
    for root in ["model", "component"] {
        let document = document(root, instances_json());
        for package in [
            json!("hrafnix.sensors"),
            json!(["hrafnix.sensors", "1"]),
            json!({}),
            json!({"name": "hrafnix.sensors"}),
            json!({"version": "1"}),
            json!({"name": "hrafnix.sensors", "version": "1", "unknown": true}),
        ] {
            let mut invalid = document.clone();
            invalid[root]["components"][0]["source"]["package"] = package;
            assert_rejected(root, &path, &serde_json::to_string(&invalid).unwrap());
        }
        for index in [0, 1] {
            for field in ["version", "source_hash"] {
                let mut invalid = document.clone();
                invalid[root]["components"][index]["source"][field] = json!("1");
                assert_rejected(root, &path, &serde_json::to_string(&invalid).unwrap());
            }
        }
        let mut invalid = document;
        invalid[root]["components"][2]["source"]["package"] =
            json!({"name": "hrafnix.sensors", "version": "1"});
        assert_rejected(root, &path, &serde_json::to_string(&invalid).unwrap());
    }
}

#[test]
fn missing_or_null_packages_are_written_without_package_or_version_fields() {
    let directory = TestDirectory::new();
    let path = directory.path.join("loose-files.json");
    for root in ["model", "component"] {
        let document = document(root, instances_json());
        for null_package in [false, true] {
            let mut value = document.clone();
            if null_package {
                value[root]["components"][1]["source"]["package"] = Value::Null;
            }
            fs::write(&path, serde_json::to_string(&value).unwrap()).unwrap();
            let instances = read(root, &path).unwrap();
            assert!(matches!(
                instances[1].source(),
                ComponentSource::File { package: None, .. }
            ));

            write(root, instances.clone(), &path).unwrap();

            assert_eq!(read(root, &path).unwrap(), instances);
            let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
            assert_eq!(written, document);
        }
    }
}

#[test]
fn components_arrays_and_all_instance_fields_are_required_and_strict() {
    let directory = TestDirectory::new();
    let path = directory.path.join("invalid.json");
    for root in ["model", "component"] {
        let document = document(root, instances_json());
        let mut missing = document.clone();
        missing[root].as_object_mut().unwrap().remove("components");
        let mut invalid_documents = vec![missing];
        let mut misplaced_rotation = document.clone();
        misplaced_rotation[root]["components"][0]["rotation"] = json!(0);
        invalid_documents.push(misplaced_rotation);
        for invalid in [
            Value::Null,
            json!(true),
            json!(1),
            json!("text"),
            json!({}),
            json!([null]),
            json!([[]]),
        ] {
            let mut value = document.clone();
            value[root]["components"] = invalid;
            invalid_documents.push(value);
        }
        for pointer in [
            "/components/0",
            "/components/0/source",
            "/components/0/position",
            "/components/0/gui_metadata",
            "/components/0/gui_metadata/dimensions",
            "/components/0/gui_metadata/ports/0",
            "/components/0/gui_metadata/ports/0/location",
            "/components/0/parameters/0",
            "/components/0/parameters/0/definition",
            "/components/2/source",
        ] {
            let value = document[root].pointer(pointer).unwrap();
            for field in value
                .as_object()
                .unwrap()
                .keys()
                .filter(|field| field.as_str() != "package")
            {
                let mut invalid = document.clone();
                invalid[root]
                    .pointer_mut(pointer)
                    .unwrap()
                    .as_object_mut()
                    .unwrap()
                    .remove(field);
                invalid_documents.push(invalid);
            }
            let mut invalid = document.clone();
            invalid[root].pointer_mut(pointer).unwrap()["unknown"] = json!(true);
            invalid_documents.push(invalid);
            for invalid_value in [Value::Null, json!(true), json!(1), json!("text"), json!([])] {
                let mut invalid = document.clone();
                *invalid[root].pointer_mut(pointer).unwrap() = invalid_value;
                invalid_documents.push(invalid);
            }
        }
        for invalid in invalid_documents {
            assert_rejected(root, &path, &serde_json::to_string(&invalid).unwrap());
        }
    }
}

#[test]
fn identifiers_tags_source_fields_geometry_and_parameters_are_validated() {
    let directory = TestDirectory::new();
    let path = directory.path.join("invalid-fields.json");
    let invalid_keys = [
        ("/components/0/id", "", "datastore_key_empty"),
        ("/components/0/id", "true", "datastore_key_reserved"),
        (
            "/components/0/id",
            "Temperature",
            "datastore_key_invalid_character",
        ),
        (
            "/components/0/id",
            "1sensor",
            "datastore_key_invalid_character",
        ),
        (
            "/components/0/id",
            "sensor-name",
            "datastore_key_invalid_character",
        ),
        (
            "/components/0/id",
            "p_sensor",
            "datastore_key_invalid_prefix",
        ),
        (
            "/components/0/parameters/0/key",
            "v_unit",
            "datastore_key_invalid_prefix",
        ),
    ];
    let mut invalid_fields = Vec::new();
    for tag in [
        json!("unknown"),
        json!("File"),
        json!(0),
        json!(1),
        json!(false),
        Value::Null,
        json!({}),
        json!([]),
    ] {
        for pointer in ["/components/0/source/type", "/components/2/source/type"] {
            invalid_fields.push((pointer, tag.clone()));
        }
    }
    for pointer in [
        "/components/0/id",
        "/components/0/source/package/name",
        "/components/0/source/package/version",
        "/components/0/source/path",
        "/components/0/source/package",
        "/components/2/source/name",
        "/components/2/source/version",
    ] {
        for invalid in [json!(1), json!(true), json!([]), json!({})] {
            invalid_fields.push((pointer, invalid));
        }
        if !pointer.ends_with("/package") {
            invalid_fields.push((pointer, Value::Null));
        }
    }
    for pointer in ["/components/0/position/x", "/components/0/position/y"] {
        for invalid in [
            json!(2_147_483_648_i64),
            json!(-2_147_483_649_i64),
            json!(1.5),
            json!(1.0),
            json!("1"),
            json!(true),
            Value::Null,
        ] {
            invalid_fields.push((pointer, invalid));
        }
    }
    for invalid in [
        json!(-90),
        json!(65536),
        json!(90.0),
        json!("90"),
        json!(true),
        Value::Null,
        json!([90]),
        json!({"degrees": 90}),
    ] {
        invalid_fields.push(("/components/0/gui_metadata/rotation", invalid));
    }
    for pointer in [
        "/components/0/gui_metadata/dimensions/width",
        "/components/0/gui_metadata/ports/0/location/x",
    ] {
        for invalid in [
            json!(-1),
            json!(65536),
            json!(0.5),
            json!(1.0),
            json!("1"),
            Value::Null,
        ] {
            invalid_fields.push((pointer, invalid));
        }
    }
    for pointer in [
        "/components/0/parameters",
        "/components/0/gui_metadata/ports",
    ] {
        for invalid in [
            Value::Null,
            json!(true),
            json!(1),
            json!("text"),
            json!({}),
            json!([null]),
            json!([[]]),
        ] {
            invalid_fields.push((pointer, invalid));
        }
    }
    invalid_fields.extend([
        ("/components/0/source/type", json!("builtin")),
        ("/components/2/source/type", json!("file")),
        ("/components/0/parameters/0/type", json!(0)),
        ("/components/0/parameters/0/type", json!("unknown")),
        ("/components/0/parameters/0/value", json!(1)),
        (
            "/components/0/parameters/0/definition/unit_family",
            json!("unknown"),
        ),
    ]);
    for root in ["model", "component"] {
        let document = document(root, instances_json());
        for (pointer, value) in &invalid_fields {
            let mut invalid = document.clone();
            *invalid[root].pointer_mut(pointer).unwrap() = value.clone();
            assert_rejected(root, &path, &serde_json::to_string(&invalid).unwrap());
        }
        for (pointer, key, message_key) in invalid_keys {
            let mut invalid = document.clone();
            *invalid[root].pointer_mut(pointer).unwrap() = json!(key);
            let invalid = serde_json::to_string(&invalid).unwrap();
            assert_rejected_with(root, &path, &invalid, message_key);
        }
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
            (format!("/{root}/components/0"), "source"),
            (format!("/{root}/components/0/gui_metadata"), "rotation"),
            (format!("/{root}/components/0"), "parameters"),
            (format!("/{root}/components/0/source"), "type"),
            (format!("/{root}/components/0/source/package"), "name"),
            (format!("/{root}/components/0/source/package"), "version"),
            (format!("/{root}/components/0/source"), "path"),
            (format!("/{root}/components/0/source"), "package"),
            (format!("/{root}/components/2/source"), "name"),
            (format!("/{root}/components/0/parameters/0"), "type"),
            (format!("/{root}/components/0/parameters/0"), "key"),
            (
                format!("/{root}/components/0/parameters/0/definition"),
                "default",
            ),
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
        write(root, instances.clone(), &path).unwrap();
        assert_eq!(read(root, &path).unwrap(), instances);
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
            "package": {"name": "", "version": "future-version"},
            "path": "../missing/component.hxc"
        });
        document[root]["components"][2]["source"] = json!({
            "type": "builtin", "name": "unknown.builtin", "version": "future-version"
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
        assert_eq!(instances[0].position(), Position::new(i32::MIN, i32::MAX));
        assert_eq!(
            instances[0]
                .parameters()
                .ordered_iter()
                .map(|(key, _)| key.as_str())
                .collect::<Vec<_>>(),
            ["p_z", "p_a"]
        );

        write(root, instances.clone(), &path).unwrap();

        assert_eq!(read(root, &path).unwrap(), instances);
        let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(written, document);
        assert_eq!(directory.entries(), ["snapshot.json"]);
    }
}

#[test]
fn instance_ports_are_capped_by_the_shared_gui_reader_and_writer() {
    let directory = TestDirectory::new();
    let path = directory.path.join("ports.json");
    for root in ["model", "component"] {
        let mut document = document(root, instances_json());
        document[root]["components"][0]["gui_metadata"]["ports"][0]["location"] =
            json!({"x": u16::MAX, "y": u16::MAX});
        fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
        let mut instances = read(root, &path).unwrap();
        assert_eq!(
            instances[0].gui_metadata().ports()[0].location(),
            PortLocation::new(120, 60)
        );
        let original_port = &instances[0].gui_metadata().ports()[0];
        let mut ports = instances[0].gui_metadata().ports().to_vec();
        ports[0] = Port::new(
            original_port.id().clone(),
            original_port.port_type(),
            PortLocation::new(u16::MAX, u16::MAX),
        );
        instances[0] = ComponentInstance::new(
            instances[0].id().clone(),
            instances[0].source().clone(),
            instances[0].position(),
            GuiMetadata::new(
                instances[0].gui_metadata().dimensions(),
                instances[0].gui_metadata().rotation(),
                instances[0].gui_metadata().icon().to_owned(),
                ports,
            ),
            instances[0].parameters().clone(),
        );

        write(root, instances, &path).unwrap();

        let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(
            written[root]["components"][0]["gui_metadata"]["ports"][0]["location"],
            json!({"x": 120, "y": 60})
        );
    }
}

#[test]
fn writing_duplicate_instance_ids_preserves_the_existing_document() {
    let directory = TestDirectory::new();
    let path = directory.path.join("duplicate-ids.json");
    for root in ["model", "component"] {
        let original = serde_json::to_string(&document(root, instances_json())).unwrap();
        fs::write(&path, &original).unwrap();
        let mut instances = read(root, &path).unwrap();
        instances[2] = ComponentInstance::new(
            instances[0].id().clone(),
            instances[2].source().clone(),
            instances[2].position(),
            instances[2].gui_metadata().clone(),
            instances[2].parameters().clone(),
        );

        let error = write(root, instances, &path).unwrap_err();

        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_duplicate_component_id"
        );
        assert_eq!(
            error.translate_data().message_params()["key"].as_str(),
            "temperature_sensor"
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), original);
        assert_eq!(directory.entries(), ["duplicate-ids.json"]);
        assert_eq!(directory.sibling_temporary_files(), Vec::<String>::new());
    }
}
