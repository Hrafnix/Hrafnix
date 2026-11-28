//! Tests for model file conversion, reading, writing, and diagnostics.

// Panicking helpers are idiomatic in tests.
#![allow(clippy::unwrap_used)]

use super::common::{self, METADATA, TRANSLATIONS, TestDirectory};

use camino::Utf8PathBuf;
use hrafnix_files::data::model::ModelFile;
use hrafnix_files::formats::model::{read_model_file, write_model_file};
use hrafnix_shareable_string::{ShareableString, SharedStringStore};
use hrafnix_translation::generate_translation_map;
use serde_json::{Value, json};
use std::fs;

/// Returns the model stored in `tests/data/minimal.hxm`.
fn minimal_model() -> ModelFile {
    read_model_file(&Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/minimal.hxm"))
        .unwrap()
}

/// Returns the fixture text with LF line endings and no final newline.
fn minimal_text() -> String {
    fs::read_to_string(Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/minimal.hxm"))
        .unwrap()
        .replace("\r\n", "\n")
        .trim_end_matches('\n')
        .to_owned()
}

#[test]
fn reading_populates_file_hash() {
    let path = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/minimal.hxm");
    let model = read_model_file(&path).unwrap();

    assert_ne!(model.file_hash(), [0; 32]);
    assert_ne!(
        model.file_hash(),
        *blake3::hash(fs::read(&path).unwrap().as_slice()).as_bytes()
    );
}

#[test]
fn version_one_document_variants_preserve_metadata() {
    common::assert_document_variants("model", |path| {
        assert_eq!(read_model_file(path)?, minimal_model());
        Ok(())
    });
}

#[test]
fn malformed_documents_report_json_categories_and_positions() {
    common::assert_malformed_documents("model", |path| read_model_file(path).map(|_| ()));
}

#[test]
fn schema_errors_include_file_diagnostic_context() {
    common::assert_schema_error_context("model", |path| read_model_file(path).map(|_| ()));
}

#[test]
fn validates_the_entire_json_document() {
    common::assert_json_trailing_content("model", |path| read_model_file(path).map(|_| ()));
}

#[test]
fn validates_utf8_before_parsing() {
    common::assert_utf8_handling("model", |path| read_model_file(path).map(|_| ()));
}

#[cfg(unix)]
#[test]
fn opening_fifo_does_not_block() {
    common::assert_fifo_rejected(|path| read_model_file(path).map(|_| ()));
}

#[cfg(unix)]
#[test]
fn saving_below_fifo_does_not_block() {
    common::assert_fifo_parent_rejected(|path| write_model_file(minimal_model(), path));
}

#[cfg(unix)]
#[test]
fn directory_access_failure_does_not_replace_existing_file() {
    common::assert_save_error_preserves_file(|path| write_model_file(minimal_model(), path));
}

#[test]
fn read_only_model_file_is_not_replaced() {
    common::assert_read_only_file_preserved(|path| write_model_file(minimal_model(), path));
}

#[test]
fn oversized_model_file_is_rejected() {
    common::assert_oversized_file_rejected(|path| read_model_file(path).map(|_| ()));
}

#[test]
fn unknown_content_is_rejected_instead_of_being_dropped() {
    let directory = TestDirectory::new();
    let path = directory.path.join("unknown.hxm");
    fs::write(
        &path,
        format!(
            "{{\"file-version\": \"1\", \"model\": {{\"metadata\": {METADATA}, \"translations\": {TRANSLATIONS}, \"foreground_decorations\": [], \"background_decorations\": [], \"global_settings\": [], \"components\": [], \"connections\": [], \"parameter_definition\": [], \"variable_definition\": [], \"unknown\": {{}}}}}}"
        ),
    )
    .unwrap();

    let error = read_model_file(&path).unwrap_err();

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_json_data"
    );
}

#[test]
fn model_round_trips_through_the_writer_and_disk() {
    let directory = TestDirectory::new();
    let source = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/minimal.hxm");
    let destination = directory.path.join("round-trip.hxm");
    let model = read_model_file(&source).unwrap();
    write_model_file(model.clone(), &destination).unwrap();

    assert_eq!(fs::read_to_string(&destination).unwrap(), minimal_text());
    assert_eq!(read_model_file(&destination).unwrap(), model);
}

#[test]
fn populated_model_sections_round_trip_through_disk() {
    let directory = TestDirectory::new();
    let path = directory.path.join("populated.hxm");
    let mut document: Value = serde_json::from_str(&minimal_text()).unwrap();
    let model = &mut document["model"];
    model["foreground_decorations"] = json!([{
        "id": "label", "position": {"x": -10, "y": 20},
        "dimensions": {"width": 100, "height": 50},
        "kind": {"Text": {"text": "Title", "font_size": 2050}}
    }]);
    model["background_decorations"] = json!([{
        "id": "shape", "position": {"x": -20, "y": -30},
        "dimensions": {"width": 120, "height": 80}, "kind": "Frame"
    }]);
    model["connections"] = common::connections_json();
    model["translations"] = json!({
        "language": "en",
        "translations": {
            "Target temperature": {"es": "Temperatura objetivo", "fr": "Temp\u{e9}rature cible"},
            "empty": {}, "quoted \"key\"\n": {"": ""}
        }
    });
    model["parameter_definition"] = json!([
        {"key": "p_label", "type": "string", "description": "Label", "default": "base"}
    ]);
    model["variable_definition"] = json!([
        {"key": "v_label", "type": "string", "description": "Output label", "default": ""}
    ]);
    model["global_settings"] = json!([
        {
            "key": "g_steps", "type": "integer",
            "definition": {"description": "Simulation steps", "default": "100", "constraint": {"type": "none"}},
            "value": "50 * 4"
        },
        {
            "key": "g_label", "type": "string",
            "definition": {"description": "Run label", "default": "\"Default\""},
            "value": "\"Custom\""
        }
    ]);
    fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
    let original = read_model_file(&path).unwrap();

    write_model_file(original.clone(), &path).unwrap();

    assert_eq!(read_model_file(&path).unwrap(), original);
    let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(written, document);
    assert_eq!(directory.entries(), ["populated.hxm"]);
}

#[test]
fn global_map_settings_round_trip_and_propagate_value_schema_errors() {
    let directory = TestDirectory::new();
    let path = directory.path.join("global-map.hxm");
    let mut document: Value = serde_json::from_str(&minimal_text()).unwrap();
    *document.pointer_mut("/model/global_settings").unwrap() = json!([{
        "key": "g_labels",
        "type": "map",
        "definition": {
            "description": "Labels",
            "items": [
                {"key": "name", "type": "string", "description": "", "default": ""}
            ],
            "default": null
        },
        "value": {"first": ["sample"]}
    }]);
    fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
    let model = read_model_file(&path).unwrap();

    write_model_file(model.clone(), &path).unwrap();

    let restored: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(restored, document);
    assert_eq!(read_model_file(&path).unwrap(), model);

    for value in [
        json!({"first": []}),
        json!({"first": ["sample", "extra"]}),
        json!({"first": [{"parameter": "", "rows": [["table instead of string"]]}]}),
    ] {
        *document
            .pointer_mut("/model/global_settings/0/value")
            .unwrap() = value;
        fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
        let error = read_model_file(&path).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "datastore_schema_mismatch",
            "{document}",
        );
    }
}

#[test]
fn writing_replaces_an_existing_model_file() {
    let directory = TestDirectory::new();
    let path = directory.path.join("existing.hxm");
    fs::write(
        &path,
        "old content that must be completely replaced".repeat(10),
    )
    .unwrap();
    let model = minimal_model();

    write_model_file(model, &path).unwrap();

    assert_eq!(fs::read_to_string(&path).unwrap(), minimal_text());
    assert_eq!(read_model_file(&path).unwrap(), minimal_model());
    assert_eq!(directory.entries(), ["existing.hxm"]);
}

#[cfg(unix)]
#[test]
fn writing_long_filenames_succeeds() {
    common::assert_long_filenames(
        "hxm",
        |path| write_model_file(minimal_model(), path),
        |path| read_model_file(path).map(|_| ()),
    );
}

#[test]
fn writing_to_a_directory_fails_and_leaves_its_contents_and_no_temporary_file() {
    let directory = TestDirectory::new();
    fs::write(directory.path.join("keep.txt"), "kept").unwrap();
    let model = minimal_model();

    let error = write_model_file(model, &directory.path).unwrap_err();

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
    let path = directory.path.join("missing").join("model.hxm");
    let model = minimal_model();
    let error = write_model_file(model, &path).unwrap_err();
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
    let path = Utf8PathBuf::from("invalid\0.hxm");
    let model = minimal_model();
    let error = write_model_file(model, &path).unwrap_err();

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_invalid_input"
    );
    assert!(error.extra_detail().is_some());
}

#[test]
fn opening_missing_model_reports_localized_io_error() {
    let path = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/missing model.hxm");
    let error = read_model_file(&path).unwrap_err();
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
    let error = read_model_file(&path).unwrap_err();
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
    let path = Utf8PathBuf::from("invalid\0.hxm");
    let error = read_model_file(&path).unwrap_err();
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
fn opening_existing_model_succeeds() {
    let path = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/minimal.hxm");

    assert_eq!(read_model_file(&path).unwrap(), minimal_model());
}
