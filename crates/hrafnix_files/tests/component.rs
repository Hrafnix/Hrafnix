//! Integration tests for component file conversion, reading, writing, and diagnostics.

// Panicking helpers are idiomatic in tests.
#![allow(clippy::indexing_slicing, clippy::unwrap_used)]

mod common;

use camino::Utf8PathBuf;
use common::TestDirectory;
use hrafnix_common::port::PortType;
use hrafnix_files::component::{data::ComponentFile, read_component_file, write_component_file};
use hrafnix_files::graphics::data::Dimension;
use hrafnix_files::gui_metadata::data::{GuiMetadata, Port, PortLocation};
use hrafnix_keys::port_key;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};
use hrafnix_translation::generate_translation_map;
use serde_json::{Value, json};
use std::fs;

/// Returns the component stored in `tests/data/minimal.hxc`.
fn minimal_component() -> ComponentFile {
    ComponentFile {
        metadata: common::metadata("component"),
        translations: common::translations(),
        gui_metadata: GuiMetadata {
            dimensions: Dimension::new(100, 50),
            icon: "../icons/component.svg".into(),
            ports: Vec::new(),
        },
        foreground_decorations: Vec::new(),
        background_decorations: Vec::new(),
        connections: Vec::new(),
    }
}

/// Returns the fixture text with LF line endings and no final newline.
fn minimal_text() -> String {
    fs::read_to_string(Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/minimal.hxc"))
        .unwrap()
        .replace("\r\n", "\n")
        .trim_end_matches('\n')
        .to_owned()
}

#[test]
fn root_accepts_named_fields_and_rejects_positional_arrays() {
    common::assert_root_structure("component", |path| read_component_file(path).map(|_| ()));
}

#[test]
fn decorations_reject_positional_arrays_and_unknown_fields() {
    common::assert_decoration_structure("component", |path| read_component_file(path).map(|_| ()));
}

#[test]
fn translations_are_required_and_validated() {
    common::assert_translation_structure("component", |path| read_component_file(path).map(|_| ()));
}

#[test]
fn connections_are_required_and_validated() {
    common::assert_connection_structure("component", |path| read_component_file(path).map(|_| ()));
}

#[test]
fn version_one_document_variants_preserve_metadata() {
    common::assert_document_variants("component", |path| {
        assert_eq!(read_component_file(path)?, minimal_component());
        Ok(())
    });
}

#[test]
fn malformed_documents_report_json_categories_and_positions() {
    common::assert_malformed_documents("component", |path| read_component_file(path).map(|_| ()));
}

#[test]
fn validates_the_document_schema_and_metadata() {
    common::assert_document_schema("component", |path| read_component_file(path).map(|_| ()));
}

#[test]
fn validates_the_entire_json_document() {
    common::assert_json_trailing_content("component", |path| read_component_file(path).map(|_| ()));
}

#[test]
fn validates_utf8_before_parsing() {
    common::assert_utf8_handling("component", |path| read_component_file(path).map(|_| ()));
}

#[cfg(unix)]
#[test]
fn opening_fifo_does_not_block() {
    common::assert_fifo_rejected(|path| read_component_file(path).map(|_| ()));
}

#[cfg(unix)]
#[test]
fn saving_below_fifo_does_not_block() {
    common::assert_fifo_parent_rejected(|path| write_component_file(minimal_component(), path));
}

#[cfg(unix)]
#[test]
fn directory_access_failure_does_not_replace_existing_file() {
    common::assert_save_error_preserves_file(|path| {
        write_component_file(minimal_component(), path)
    });
}

#[test]
fn read_only_component_file_is_not_replaced() {
    common::assert_read_only_file_preserved(|path| write_component_file(minimal_component(), path));
}

#[test]
fn oversized_component_file_is_rejected() {
    common::assert_oversized_file_rejected(|path| read_component_file(path).map(|_| ()));
}

#[test]
fn unknown_content_is_rejected_instead_of_being_dropped() {
    let directory = TestDirectory::new();
    let path = directory.path.join("unknown.hxc");
    let mut document: Value = serde_json::from_str(&minimal_text()).unwrap();
    document["component"]["unknown"] = json!({});
    fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();

    let error = read_component_file(&path).unwrap_err();

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_json_data"
    );
}

#[test]
fn component_round_trips_through_the_writer_and_disk() {
    let directory = TestDirectory::new();
    let source = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/minimal.hxc");
    let destination = directory.path.join("round-trip.hxc");
    let component = read_component_file(&source).unwrap();
    write_component_file(component.clone(), &destination).unwrap();

    assert_eq!(fs::read_to_string(&destination).unwrap(), minimal_text());
    assert_eq!(read_component_file(&destination).unwrap(), component);
}

#[test]
fn gui_metadata_is_required_and_validated() {
    let directory = TestDirectory::new();
    let path = directory.path.join("gui-metadata.hxc");
    let document: Value = serde_json::from_str(&minimal_text()).unwrap();
    let mut missing = document.clone();
    missing["component"]
        .as_object_mut()
        .unwrap()
        .remove("gui_metadata");
    let mut invalid_documents = vec![missing];
    for invalid in [
        Value::Null,
        json!([]),
        json!([{"width": 100, "height": 50}, "icon.svg", []]),
        json!({}),
        json!({"dimensions": [100, 50], "icon": "icon.svg", "ports": []}),
        json!({"dimensions": {"width": 100, "height": 50}, "ports": []}),
        json!({"dimensions": {"width": 100, "height": 50}, "icon": false, "ports": []}),
        json!({"dimensions": {"width": 100, "height": 50}, "icon": "", "ports": [], "extra": 1}),
        json!({"dimensions": {"width": 100, "height": 50}, "icon": ""}),
        json!({"dimensions": {"width": 100, "height": 50}, "icon": "", "ports": null}),
    ] {
        let mut invalid_document = document.clone();
        invalid_document["component"]["gui_metadata"] = invalid;
        invalid_documents.push(invalid_document);
    }
    for invalid_document in invalid_documents {
        fs::write(&path, serde_json::to_string(&invalid_document).unwrap()).unwrap();
        let error = read_component_file(&path).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_json_data",
            "{invalid_document}",
        );
        assert!(error.extra_detail().is_some());
    }
}

#[test]
fn component_gui_metadata_round_trips_through_disk() {
    let directory = TestDirectory::new();
    let path = directory.path.join("gui-metadata.hxc");
    for icon in [
        "",
        "../icons/missing \u{3b1}.svg",
        " leading and trailing spaces ",
        r"C:\icons\missing.png",
    ] {
        let mut component = minimal_component();
        component.gui_metadata = GuiMetadata {
            dimensions: Dimension::new(0, u16::MAX),
            icon: icon.into(),
            ports: Vec::new(),
        };
        write_component_file(component.clone(), &path).unwrap();
        assert_eq!(read_component_file(&path).unwrap(), component);
        let document: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(
            document["component"]["gui_metadata"],
            json!({
                "dimensions": {"width": 0, "height": u16::MAX},
                "icon": icon,
                "ports": [],
            }),
        );
    }
}

#[test]
fn component_ports_round_trip_inside_gui_metadata() {
    let directory = TestDirectory::new();
    let path = directory.path.join("ports.hxc");
    let mut component = minimal_component();
    component.gui_metadata.ports = vec![
        Port {
            id: port_key!("temperature").into(),
            port_type: PortType::Output,
            location: PortLocation { x: 100, y: 25 },
        },
        Port {
            id: port_key!("power").into(),
            port_type: PortType::Power,
            location: PortLocation { x: 0, y: 50 },
        },
    ];
    write_component_file(component.clone(), &path).unwrap();
    assert_eq!(read_component_file(&path).unwrap(), component);
    let document: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(
        document["component"]["gui_metadata"]["ports"],
        json!([
            {"id": "temperature", "type": "output", "location": {"x": 100, "y": 25}},
            {"id": "power", "type": "power", "location": {"x": 0, "y": 50}},
        ]),
    );
    assert!(document["component"].get("ports").is_none());
}

#[test]
fn reading_caps_ports_to_the_icon_bounds() {
    let directory = TestDirectory::new();
    let path = directory.path.join("ports.hxc");
    for (x, y) in [(101, 25), (100, 51), (u16::MAX, u16::MAX)] {
        let mut document: Value = serde_json::from_str(&minimal_text()).unwrap();
        document["component"]["gui_metadata"]["ports"] = json!([{
            "id": "temperature",
            "type": "output",
            "location": {"x": x, "y": y},
        }]);
        fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
        let component = read_component_file(&path).unwrap();
        assert_eq!(
            component.gui_metadata.ports,
            vec![Port {
                id: port_key!("temperature").into(),
                port_type: PortType::Output,
                location: PortLocation {
                    x: x.min(100),
                    y: y.min(50),
                },
            }],
        );
        write_component_file(component.clone(), &path).unwrap();
        assert_eq!(read_component_file(&path).unwrap(), component);
        let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(
            written["component"]["gui_metadata"]["ports"][0]["location"],
            json!({"x": x.min(100), "y": y.min(50)}),
        );
    }
}

#[test]
fn writing_caps_port_locations_when_replacing_existing_files() {
    let directory = TestDirectory::new();
    let path = directory.path.join("ports.hxc");
    let original = minimal_text();
    fs::write(&path, &original).unwrap();
    for (x, y) in [(101, 25), (100, 51), (u16::MAX, u16::MAX)] {
        let mut component = minimal_component();
        component.gui_metadata.ports.push(Port {
            id: port_key!("temperature").into(),
            port_type: PortType::Output,
            location: PortLocation { x, y },
        });
        let mut expected = component.clone();
        expected.gui_metadata.ports[0].location = PortLocation {
            x: x.min(100),
            y: y.min(50),
        };
        write_component_file(component, &path).unwrap();
        assert_eq!(read_component_file(&path).unwrap(), expected);
        let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(
            written["component"]["gui_metadata"]["ports"][0]["location"],
            json!({"x": x.min(100), "y": y.min(50)}),
        );
        assert_eq!(directory.entries(), ["ports.hxc"]);
        assert_eq!(directory.sibling_temporary_files(), Vec::<String>::new());
    }
}

#[test]
fn component_decorations_round_trip_through_disk() {
    let directory = TestDirectory::new();
    let path = directory.path.join("decorations.hxc");
    let mut component = minimal_component();
    component
        .foreground_decorations
        .push(common::foreground_decoration());
    component
        .background_decorations
        .push(common::background_decoration());

    write_component_file(component.clone(), &path).unwrap();

    assert_eq!(read_component_file(&path).unwrap(), component);
}

#[test]
fn component_connections_round_trip_through_disk_without_style() {
    let directory = TestDirectory::new();
    let path = directory.path.join("connections.hxc");
    let mut component = minimal_component();
    component.connections = common::connections();

    write_component_file(component.clone(), &path).unwrap();

    assert_eq!(read_component_file(&path).unwrap(), component);
    let document: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(
        document["component"]["connections"],
        common::connections_json(),
    );
}

#[test]
fn component_translations_round_trip_through_disk() {
    let directory = TestDirectory::new();
    let path = directory.path.join("translations.hxc");
    let mut component = minimal_component();
    component.translations = common::populated_translations();

    write_component_file(component.clone(), &path).unwrap();

    assert_eq!(read_component_file(&path).unwrap(), component);
    let document: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(
        document["component"]["translations"],
        json!({
            "language": "en",
            "translations": {
                "Target temperature": {
                    "es": "Temperatura objetivo",
                    "fr": "Temp\u{e9}rature cible",
                    "de": "Zieltemperatur",
                },
                "empty": {},
                "quoted \"key\"\n": {"": ""},
            },
        }),
    );
}

#[test]
fn writing_replaces_an_existing_component_file() {
    let directory = TestDirectory::new();
    let path = directory.path.join("existing.hxc");
    fs::write(
        &path,
        "old content that must be completely replaced".repeat(10),
    )
    .unwrap();
    let component = minimal_component();

    write_component_file(component, &path).unwrap();

    assert_eq!(fs::read_to_string(&path).unwrap(), minimal_text());
    assert_eq!(read_component_file(&path).unwrap(), minimal_component());
    assert_eq!(directory.entries(), ["existing.hxc"]);
}

#[cfg(unix)]
#[test]
fn writing_long_filenames_succeeds() {
    common::assert_long_filenames(
        "hxc",
        |path| write_component_file(minimal_component(), path),
        |path| read_component_file(path).map(|_| ()),
    );
}

#[test]
fn writing_to_a_directory_fails_and_leaves_its_contents_and_no_temporary_file() {
    let directory = TestDirectory::new();
    fs::write(directory.path.join("keep.txt"), "kept").unwrap();
    let component = minimal_component();

    let error = write_component_file(component, &directory.path).unwrap_err();

    assert_eq!(
        error.category(),
        hrafnix_message::message::MessageCategory::File
    );
    // Renaming a file over a directory yields EISDIR on Unix; Windows reports access denied.
    #[cfg(unix)]
    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_not_a_file"
    );
    assert!(directory.path.is_dir());
    assert_eq!(directory.entries(), ["keep.txt"]);
    assert_eq!(directory.sibling_temporary_files(), Vec::<String>::new());
}

#[test]
fn writing_to_a_missing_parent_reports_localized_io_error() {
    let directory = TestDirectory::new();
    let path = directory.path.join("missing").join("component.hxc");
    let component = minimal_component();
    let error = write_component_file(component, &path).unwrap_err();
    let map = generate_translation_map(&SharedStringStore::new());

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_not_found"
    );
    assert_eq!(
        error
            .translated_message(&map, "en")
            .as_ref()
            .map(ShareableString::as_str),
        Some(format!("File or directory not found: '{path}'").as_str())
    );
    assert!(error.extra_detail().is_some());
    assert!(!path.exists());
}

#[test]
fn writing_to_an_invalid_path_reports_localized_io_error() {
    let path = Utf8PathBuf::from("invalid\0.hxc");
    let component = minimal_component();
    let error = write_component_file(component, &path).unwrap_err();

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_invalid_input"
    );
    assert!(error.extra_detail().is_some());
}

#[test]
fn opening_missing_component_reports_localized_io_error() {
    let path =
        Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/missing component.hxc");
    let error = read_component_file(&path).unwrap_err();
    let map = generate_translation_map(&SharedStringStore::new());

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_not_found"
    );
    assert_eq!(
        error
            .translated_message(&map, "en")
            .as_ref()
            .map(ShareableString::as_str),
        Some(format!("File or directory not found: '{path}'").as_str())
    );
    assert_eq!(
        error
            .translated_message(&map, "de")
            .as_ref()
            .map(ShareableString::as_str),
        Some(format!("Datei oder Verzeichnis nicht gefunden: '{path}'").as_str())
    );
    assert!(error.extra_detail().is_some());
}

#[test]
fn opening_directory_reports_not_a_file() {
    let path = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    let error = read_component_file(&path).unwrap_err();
    let map = generate_translation_map(&SharedStringStore::new());

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_not_a_file"
    );
    assert_eq!(
        error
            .translated_message(&map, "en")
            .as_ref()
            .map(ShareableString::as_str),
        Some(format!("Path is not a file: '{path}'").as_str())
    );
}

#[test]
fn invalid_path_reports_localized_io_error_without_os_text() {
    let path = Utf8PathBuf::from("invalid\0.hxc");
    let error = read_component_file(&path).unwrap_err();
    let map = generate_translation_map(&SharedStringStore::new());

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_invalid_input"
    );
    assert_eq!(
        error
            .translated_message(&map, "en")
            .as_ref()
            .map(ShareableString::as_str),
        Some(format!("Invalid path or file operation argument: '{path}'").as_str())
    );
    assert!(error.extra_detail().is_some());
}

#[test]
fn opening_existing_component_succeeds() {
    let path = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/minimal.hxc");

    assert_eq!(read_component_file(&path).unwrap(), minimal_component());
}
