//! Shared support for file integration tests.

use camino::{Utf8Path, Utf8PathBuf};
use hrafnix_files::background_decoration::data::{BackgroundDecoration, BackgroundDecorationKind};
use hrafnix_files::foreground_decoration::data::{
    ForegroundDecoration, ForegroundDecorationKind, TextDecoration,
};
use hrafnix_files::graphics::data::{Dimension, FontSize, Position};
use hrafnix_files::metadata::data::MetadataFile;
use hrafnix_message::message::Message;
use serde_json::{Value, json};
use std::fs;
use std::io::ErrorKind;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Maximum supported file size, in bytes (256 MiB).
const MAX_FILE_SIZE: u64 = 256 * 1024 * 1024;

/// Valid version 1 metadata entry.
pub(super) const METADATA: &str = "{\"display-name\": \"\", \"description\": \"\", \"author\": \"\", \"minimum-version-major\": 0, \"minimum-version-minor\": 0}";

/// Valid version 1 contents of a model or component root entry.
pub(super) const CONTENTS: &str = "{\"metadata\": {\"display-name\": \"\", \"description\": \"\", \"author\": \"\", \"minimum-version-major\": 0, \"minimum-version-minor\": 0}, \"foreground_decorations\": [], \"background_decorations\": []}";

/// Returns a text decoration for document-level round trips.
pub(super) fn foreground_decoration() -> ForegroundDecoration {
    ForegroundDecoration {
        id: "label".into(),
        position: Position::new(-10, 20),
        dimensions: Dimension::new(100, 50),
        kind: ForegroundDecorationKind::Text(TextDecoration {
            text: "Title".into(),
            font_size: FontSize::new(2050),
        }),
    }
}

/// Returns a frame for document-level round trips.
pub(super) fn background_decoration() -> BackgroundDecoration {
    BackgroundDecoration {
        id: "shape".into(),
        position: Position::new(-20, -30),
        dimensions: Dimension::new(120, 80),
        kind: BackgroundDecorationKind::Frame,
    }
}

/// Returns the metadata stored in the `minimal` test data files for `kind`.
pub(super) fn metadata(kind: &str) -> MetadataFile {
    MetadataFile {
        display_name: "Example".into(),
        description: format!("An example {kind}.").into(),
        author: "Hrafnix".into(),
        minimum_version_major: 1,
        minimum_version_minor: 2,
    }
}

/// An isolated directory removed when its test finishes.
#[derive(Debug)]
pub(super) struct TestDirectory {
    /// The directory owned by this test.
    pub(super) path: Utf8PathBuf,
}

impl TestDirectory {
    /// Creates a unique directory without replacing any existing path.
    pub(super) fn new() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

        loop {
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            let path = Utf8PathBuf::try_from(std::env::temp_dir())
                .unwrap()
                .join(format!("hrafnix-files-test-{}-{id}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Self { path },
                Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
                Err(error) => panic!("Failed to create test directory '{path}': {error}"),
            }
        }
    }

    /// Returns the sorted names of every entry in this directory.
    pub(super) fn entries(&self) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(&self.path)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }

    /// Returns the names of leftover temporary files written beside this directory.
    pub(super) fn sibling_temporary_files(&self) -> Vec<String> {
        let prefix = format!(".{}.", self.path.file_name().unwrap());
        let mut names: Vec<String> = fs::read_dir(self.path.parent().unwrap())
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|name| name.starts_with(&prefix))
            .collect();
        names.sort();
        names
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        // Never panic during cleanup: a failing test is already unwinding.
        drop(fs::remove_dir_all(&self.path));
    }
}

/// Checks creation and replacement at the common 255-byte filename limit.
#[cfg(unix)]
pub(super) fn assert_long_filenames(
    extension: &str,
    write: fn(&Utf8Path) -> Result<(), Message>,
    read: fn(&Utf8Path) -> Result<(), Message>,
) {
    for stem in ["a".repeat(251), format!("{}ab", "界".repeat(83))] {
        let directory = TestDirectory::new();
        let name = format!("{stem}.{extension}");
        assert_eq!(name.len(), 255);
        let path = directory.path.join(&name);

        write(&path).unwrap();
        read(&path).unwrap();
        fs::write(&path, "original contents").unwrap();
        write(&path).unwrap();
        read(&path).unwrap();
        assert_eq!(directory.entries(), [name]);
    }
}

/// Checks named struct fields while rejecting positional arrays and invalid root values.
pub(super) fn assert_root_structure(root: &str, read: fn(&Utf8Path) -> Result<(), Message>) {
    let directory = TestDirectory::new();
    let path = directory.path.join("root.json");
    fs::write(
        &path,
        format!("{{\"file-version\":\"1\",\"{root}\":{CONTENTS}}}"),
    )
    .unwrap();
    read(&path).unwrap();

    let invalid_bodies = [
        format!("[{METADATA}]"),
        "{\"metadata\":[\"\", \"\", \"\", 0, 0],\"foreground_decorations\":[],\"background_decorations\":[]}".to_owned(),
        "[[\"\", \"\", \"\", 0, 0]]".to_owned(),
        "[]".to_owned(),
        "[{}]".to_owned(),
        "[1]".to_owned(),
        "[[]]".to_owned(),
        "null".to_owned(),
        "true".to_owned(),
        "1".to_owned(),
        "\"text\"".to_owned(),
    ];
    let documents = invalid_bodies
        .iter()
        .map(|body| format!("{{\"file-version\":\"1\",\"{root}\":{body}}}"))
        .chain([format!("[\"1\",{CONTENTS}]")]);
    for document in documents {
        fs::write(&path, &document).unwrap();
        let error = read(&path).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_json_data",
            "{document}"
        );
        assert_eq!(
            error
                .translate_data()
                .message_params()
                .get("path")
                .map(hrafnix_shareable_string::ShareableString::as_str),
            Some(path.as_str()),
            "{document}"
        );
        assert!(error.extra_detail().is_some(), "{document}");
    }
}

/// Checks that decoration structs require JSON objects and reject unknown fields.
#[allow(
    clippy::indexing_slicing,
    reason = "Tests deliberately mutate known-valid JSON fixtures."
)]
pub(super) fn assert_decoration_structure(root: &str, read: fn(&Utf8Path) -> Result<(), Message>) {
    let directory = TestDirectory::new();
    let path = directory.path.join("decorations.json");
    let mut document: Value =
        serde_json::from_str(&format!("{{\"file-version\":\"1\",\"{root}\":{CONTENTS}}}")).unwrap();
    let position = json!({"x": -10, "y": 20});
    let dimensions = json!({"width": 100, "height": 50});
    let foreground = [
        json!({
            "id": "text",
            "position": position,
            "dimensions": dimensions,
            "kind": {"Text": {
                "text": "Title",
                "font_size": 2050,
            }},
        }),
        json!({
            "id": "image",
            "position": position,
            "dimensions": dimensions,
            "kind": {"Image": {"path": "image.png", "fit": "Contain"}},
        }),
    ];
    let background = [json!({
        "id": "frame",
        "position": position,
        "dimensions": dimensions,
        "kind": "Frame",
    })];
    document[root]["foreground_decorations"] = json!(foreground);
    document[root]["background_decorations"] = json!(background);
    fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
    read(&path).unwrap();

    let mut invalid_documents = Vec::new();
    for (field, decorations) in [
        ("foreground_decorations", foreground.as_slice()),
        ("background_decorations", background.as_slice()),
    ] {
        for (index, decoration) in decorations.iter().enumerate() {
            let mut invalid = document.clone();
            invalid[root][field][index] = json!([
                decoration["id"],
                decoration["position"],
                decoration["dimensions"],
                decoration["kind"],
            ]);
            invalid_documents.push(invalid);

            let mut invalid = document.clone();
            invalid[root][field][index]["unknown"] = json!(true);
            invalid_documents.push(invalid);

            for (property, array) in [
                ("position", json!([-10, 20])),
                ("dimensions", json!([100, 50])),
            ] {
                let mut invalid = document.clone();
                invalid[root][field][index][property] = array;
                invalid_documents.push(invalid);

                let mut invalid = document.clone();
                invalid[root][field][index][property]["unknown"] = json!(true);
                invalid_documents.push(invalid);
            }

            let Some(kind_object) = decoration["kind"].as_object() else {
                for kind in [
                    json!("Unknown"),
                    json!(["Frame"]),
                    json!({"Frame": null}),
                    json!({"Frame": {}}),
                ] {
                    let mut invalid = document.clone();
                    invalid[root][field][index]["kind"] = kind;
                    invalid_documents.push(invalid);
                }
                continue;
            };
            let (kind, payload) = kind_object.iter().next().unwrap();
            let properties: &[&str] = match kind.as_str() {
                "Text" => &["text", "font_size"],
                "Image" => &["path", "fit"],
                _ => panic!("unexpected decoration kind: {kind}"),
            };
            let mut invalid = document.clone();
            invalid[root][field][index]["kind"][kind] = json!(
                properties
                    .iter()
                    .map(|property| payload[property].clone())
                    .collect::<Vec<_>>()
            );
            invalid_documents.push(invalid);

            let mut invalid = document.clone();
            invalid[root][field][index]["kind"]["unknown"] = json!(true);
            invalid_documents.push(invalid);

            let mut invalid = document.clone();
            invalid[root][field][index]["kind"][kind]["unknown"] = json!(true);
            invalid_documents.push(invalid);

            for property in ["font_size"] {
                if let Some(value) = payload.get(property) {
                    for wrapped in [json!([value]), json!({"value": value})] {
                        let mut invalid = document.clone();
                        invalid[root][field][index]["kind"][kind][property] = wrapped;
                        invalid_documents.push(invalid);
                    }
                }
            }
        }
    }

    for document in invalid_documents {
        fs::write(&path, serde_json::to_string(&document).unwrap()).unwrap();
        let Err(error) = read(&path) else {
            panic!("accepted invalid decoration: {document}");
        };
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_json_data",
            "{document}"
        );
        assert!(error.extra_detail().is_some(), "{document}");
    }
}

/// Checks supported formatting, field order, and escaped root names.
pub(super) fn assert_document_variants(root: &str, read: fn(&Utf8Path) -> Result<(), Message>) {
    let directory = TestDirectory::new();
    let path = directory.path.join("variants.json");
    let contents = format!(
        "{{\"metadata\":{{\"display-name\":\"Example\",\"description\":\"An example {root}.\",\"author\":\"Hrafnix\",\"minimum-version-major\":1,\"minimum-version-minor\":2}},\"foreground_decorations\":[],\"background_decorations\":[]}}"
    );
    let escaped_root = root.replace('o', "\\u006f");
    for document in [
        format!("{{\"file-version\":\"1\",\"{root}\":{contents}}}"),
        format!("{{\"{root}\":{contents},\"file-version\":\"1\"}}"),
        format!("{{\n  \"file-version\": \"1\",\n  \"{root}\": {contents}\n}}\n"),
        format!(" \t\r\n{{ \"file-version\" : \"1\" , \"{root}\" : {contents} }} \t\r\n"),
        format!("\u{feff}{{\"file-version\":\"1\",\"{root}\":{contents}}}"),
        format!("{{\"file-version\":\"1\",\"{escaped_root}\":{contents}}}"),
    ] {
        fs::write(&path, &document).unwrap();
        read(&path).unwrap_or_else(|error| panic!("{document}: {error:?}"));
    }
}

/// Checks JSON syntax and EOF diagnostics, including their exact parser positions.
pub(super) fn assert_malformed_documents(root: &str, read: fn(&Utf8Path) -> Result<(), Message>) {
    let directory = TestDirectory::new();
    let path = directory.path.join("malformed.json");
    for (document, key, line, column) in [
        (String::new(), "files_json_unexpected_eof", "1", "0"),
        (" \n".to_owned(), "files_json_unexpected_eof", "2", "0"),
        ("{".to_owned(), "files_json_unexpected_eof", "1", "1"),
        (
            "{\"file-version\": \"1\"".to_owned(),
            "files_json_unexpected_eof",
            "1",
            "20",
        ),
        (
            format!("{{\"file-version\":\"1\",\"{root}\":{{\n\"metadata\":"),
            "files_json_unexpected_eof",
            "2",
            "11",
        ),
        ("not JSON".to_owned(), "files_json_syntax", "1", "2"),
        (
            "{\"file-version\": \"1\",}".to_owned(),
            "files_json_syntax",
            "1",
            "22",
        ),
        (
            "{\"file-version\": 01}".to_owned(),
            "files_json_syntax",
            "1",
            "19",
        ),
        (
            "{\"file-version\": +1}".to_owned(),
            "files_json_syntax",
            "1",
            "18",
        ),
        (
            "{'file-version': \"1\"}".to_owned(),
            "files_json_syntax",
            "1",
            "2",
        ),
        (
            "\u{a0}{\"file-version\":\"1\"}".to_owned(),
            "files_json_syntax",
            "1",
            "1",
        ),
        (
            format!("\u{feff}\u{feff}{{\"file-version\":\"1\",\"{root}\":{CONTENTS}}}"),
            "files_json_syntax",
            "1",
            "1",
        ),
    ] {
        fs::write(&path, &document).unwrap();
        let error = read(&path).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            key,
            "{document}"
        );
        for (name, expected) in [("path", path.as_str()), ("line", line), ("column", column)] {
            assert_eq!(
                error
                    .translate_data()
                    .message_params()
                    .get(name)
                    .map(hrafnix_shareable_string::ShareableString::as_str),
                Some(expected),
                "{document}: {name}"
            );
        }
        assert!(
            error.extra_detail().is_some_and(|detail| detail
                .as_str()
                .ends_with(&format!("at line {line} column {column}"))),
            "{document}: {:?}",
            error.extra_detail()
        );
    }
}

/// Checks the versioned envelope and rejects invalid metadata.
pub(super) fn assert_document_schema(root: &str, read: fn(&Utf8Path) -> Result<(), Message>) {
    let directory = TestDirectory::new();
    let path = directory.path.join("schema.json");
    fs::write(
        &path,
        format!("{{\"{root}\":{CONTENTS},\"file-version\":\"1\"}}"),
    )
    .unwrap();
    read(&path).unwrap();

    let other_root = if root == "model" {
        "component"
    } else {
        "model"
    };
    let mut invalid_documents = vec![
        "[]".to_owned(),
        format!("[{{\"file-version\":\"1\",\"{root}\":{CONTENTS}}}]"),
        "\"text\"".to_owned(),
        "1".to_owned(),
        "true".to_owned(),
        "null".to_owned(),
        "{}".to_owned(),
        format!("{{\"{root}\":{CONTENTS}}}"),
        "{\"file-version\":\"1\"}".to_owned(),
        format!("{{\"file-version\":\"1\",\"{other_root}\":{CONTENTS}}}"),
        format!("{{\"file-version\":\"1\",\"wrapper\":{{\"{root}\":{CONTENTS}}}}}"),
        format!(
            "{{\"file-version\":\"1\",\"{}\":{CONTENTS}}}",
            root.to_uppercase()
        ),
        format!("{{\"FILE-VERSION\":\"1\",\"{root}\":{CONTENTS}}}"),
        format!("{{\"file_version\":\"1\",\"{root}\":{CONTENTS}}}"),
        format!("{{\"{root}\":{{\"file-version\":\"1\"}}}}"),
        format!("{{\"file-version\":\"1\",\"{root}\":{CONTENTS},\"unknown\":{{}}}}"),
        format!("{{\"unknown\":null,\"file-version\":\"1\",\"{root}\":{CONTENTS}}}"),
        format!("{{\"\":1,\"file-version\":\"1\",\"{root}\":{CONTENTS}}}"),
        format!("{{\"file-version\":\"1\",\"file-version\":\"1\",\"{root}\":{CONTENTS}}}"),
        format!("{{\"file-version\":\"1\",\"{root}\":{CONTENTS},\"file-version\":\"1\"}}"),
        format!("{{\"file-version\":\"1\",\"{root}\":{CONTENTS},\"{root}\":{CONTENTS}}}"),
        format!("{{\"file-version\":\"1\",\"{root}\":{{}}}}"),
        format!(
            "{{\"file-version\":\"1\",\"{root}\":{{\"metadata\":{METADATA},\"metadata\":{METADATA},\"foreground_decorations\":[],\"background_decorations\":[]}}}}"
        ),
        format!(
            "{{\"file-version\":\"1\",\"{root}\":{{\"metadata\":{METADATA},\"foreground_decorations\":[],\"background_decorations\":[],\"file-version\":\"1\"}}}}"
        ),
    ];
    for version in [
        "1",
        "1.0",
        "1e0",
        "-1",
        "-0",
        "4294967296",
        "18446744073709551616",
        "null",
        "true",
        "[]",
        "[1]",
        "{}",
        "\"0\"",
        "\"2\"",
        "\"\"",
        "\"01\"",
    ] {
        invalid_documents.push(format!(
            "{{\"file-version\":{version},\"{root}\":{CONTENTS}}}"
        ));
    }
    for field in ["foreground_decorations", "background_decorations"] {
        let contents = CONTENTS.replace(&format!(", \"{field}\": []"), "");
        assert_ne!(contents, CONTENTS);
        invalid_documents.push(format!("{{\"file-version\":\"1\",\"{root}\":{contents}}}"));
        for value in ["null", "{}", "true", "1", "\"text\"", "[null]", "[{}]"] {
            let contents = CONTENTS.replace(
                &format!("\"{field}\": []"),
                &format!("\"{field}\": {value}"),
            );
            assert_ne!(contents, CONTENTS);
            invalid_documents.push(format!("{{\"file-version\":\"1\",\"{root}\":{contents}}}"));
        }
    }
    for metadata in [
        "[]".to_owned(),
        "null".to_owned(),
        "{}".to_owned(),
        METADATA.replace(
            "\"minimum-version-major\": 0",
            "\"minimum-version-major\": -1",
        ),
        METADATA.replace(
            "\"minimum-version-major\": 0",
            "\"minimum-version-major\": 65536",
        ),
        METADATA.replace(
            "\"minimum-version-minor\": 0",
            "\"minimum-version-minor\": 65536",
        ),
        METADATA.replace(
            "\"minimum-version-minor\": 0",
            "\"minimum-version-minor\": -1",
        ),
        METADATA.replace(
            "\"minimum-version-major\": 0",
            "\"minimum-version-major\": 1.5",
        ),
        METADATA.replace(
            "\"minimum-version-minor\": 0",
            "\"minimum-version-minor\": \"0\"",
        ),
        METADATA.replace("\"display-name\": \"\"", "\"display-name\": 1"),
        METADATA.replace("\"description\": \"\"", "\"description\": null"),
        METADATA.replace("\"author\": \"\"", "\"author\": true"),
        METADATA.replace("\"display-name\"", "\"displayName\""),
        METADATA.replace(", \"minimum-version-minor\": 0", ""),
        METADATA.replace(
            "\"display-name\": \"\"",
            "\"display-name\": \"\", \"display-name\": \"\"",
        ),
        METADATA.replace('}', ", \"unknown\": true}"),
    ] {
        invalid_documents.push(format!(
            "{{\"file-version\":\"1\",\"{root}\":{{\"metadata\":{metadata},\"foreground_decorations\":[],\"background_decorations\":[]}}}}"
        ));
    }
    for document in invalid_documents {
        fs::write(&path, &document).unwrap();
        let error = read(&path).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_json_data",
            "{document}"
        );
        assert_eq!(
            error
                .translate_data()
                .message_params()
                .get("path")
                .unwrap()
                .as_str(),
            path.as_str(),
            "{document}"
        );
        for name in ["line", "column"] {
            assert!(
                error.translate_data().message_params().contains_key(name),
                "{document}: {name}"
            );
        }
        assert!(error.extra_detail().is_some(), "{document}");
    }
}

/// Checks rejection of anything after the top-level object while allowing JSON whitespace.
pub(super) fn assert_json_trailing_content(root: &str, read: fn(&Utf8Path) -> Result<(), Message>) {
    let directory = TestDirectory::new();
    let path = directory.path.join("trailing.json");
    for document in [
        format!("{{\"file-version\": \"1\", \"{root}\": {CONTENTS}}}"),
        format!("{{\n  \"file-version\": \"1\",\n  \"{root}\": {CONTENTS}\n}}"),
    ] {
        for suffix in ["", " \t\r\n", "\n\n"] {
            fs::write(&path, format!("{document}{suffix}")).unwrap();
            read(&path).unwrap();
        }
        for (suffix, key) in [
            ("{}", "files_json_syntax"),
            ("garbage", "files_json_syntax"),
            (",", "files_json_syntax"),
            ("}", "files_json_syntax"),
            ("null", "files_json_syntax"),
            ("// comment", "files_json_syntax"),
            ("\u{a0}", "files_json_syntax"),
            ("\u{feff}", "files_json_syntax"),
            ("\"", "files_json_syntax"),
            ("{\"file-version\": \"1\"}", "files_json_syntax"),
        ] {
            fs::write(&path, format!("{document} \n{suffix}")).unwrap();
            let error = read(&path).unwrap_err();
            assert_eq!(
                error.translate_data().message_key().as_str(),
                key,
                "{suffix}"
            );
            assert_eq!(
                error
                    .translate_data()
                    .message_params()
                    .get("path")
                    .unwrap()
                    .as_str(),
                path.as_str()
            );
            assert_eq!(
                error
                    .translate_data()
                    .message_params()
                    .get("line")
                    .unwrap()
                    .as_str(),
                document.lines().count().checked_add(1).unwrap().to_string(),
                "{suffix}"
            );
        }
    }
}

/// Checks that a UTF-8 BOM is accepted while invalid UTF-8 is rejected with its byte
/// position before any JSON is parsed.
pub(super) fn assert_utf8_handling(root: &str, read: fn(&Utf8Path) -> Result<(), Message>) {
    use hrafnix_message::message::{MessageCategory, MessageLevel};
    use hrafnix_shareable_string::SharedStringStore;
    use hrafnix_translation::generate_translation_map;

    let directory = TestDirectory::new();
    let path = directory.path.join("encoding.json");
    let document = format!("{{\"file-version\": \"1\", \"{root}\": {CONTENTS}}}");

    fs::write(&path, format!("\u{feff}{document}")).unwrap();
    read(&path).unwrap();

    let invalid = [
        b"{\"\xF0\x90\x80\": 1,".as_slice(),
        document.strip_prefix('{').unwrap().as_bytes(),
    ]
    .concat();
    let truncated = [document.as_bytes(), b"\n\xE2\x82".as_slice()].concat();
    let truncated_at = document.len().checked_add(1).unwrap().to_string();
    let map = generate_translation_map(&SharedStringStore::new());
    for (bytes, key, params, english) in [
        (
            invalid,
            "files_invalid_utf8",
            vec![("position", "2".to_owned()), ("length", "3".to_owned())],
            format!("'{path}': Invalid UTF-8 at byte 2 (invalid sequence length: 3)"),
        ),
        (
            truncated,
            "files_incomplete_utf8",
            vec![("position", truncated_at.clone())],
            format!("'{path}': Incomplete UTF-8 sequence at byte {truncated_at}"),
        ),
    ] {
        let diagnostic = String::from_utf8(bytes.clone())
            .unwrap_err()
            .utf8_error()
            .to_string();
        fs::write(&path, bytes).unwrap();
        let error = read(&path).unwrap_err();
        assert_eq!(error.level(), MessageLevel::Error);
        assert_eq!(error.category(), MessageCategory::File);
        assert_eq!(error.translate_data().message_key().as_str(), key);
        assert_eq!(
            error.translate_data().message_params().len(),
            params.len().checked_add(1).unwrap()
        );
        assert_eq!(
            error
                .translate_data()
                .message_params()
                .get("path")
                .unwrap()
                .as_str(),
            path.as_str()
        );
        for (name, value) in params {
            assert_eq!(
                error
                    .translate_data()
                    .message_params()
                    .get(name)
                    .unwrap()
                    .as_str(),
                value,
                "{key}: {name}"
            );
        }
        assert_eq!(error.extra_detail().unwrap().as_str(), diagnostic);
        assert_eq!(
            error.translated_message(&map, "en").unwrap().as_str(),
            english
        );
        for language in ["en", "zh", "de", "es", "fr", "ja", "ko"] {
            let translated = error.translated_message(&map, language).unwrap();
            assert!(!translated.as_str().contains("%{"), "{key}: {language}");
            for value in error.translate_data().message_params().values() {
                assert!(
                    translated.as_str().contains(value.as_str()),
                    "{key}: {language}: {value}"
                );
            }
            if language != "en" {
                assert_ne!(translated.as_str(), english, "{key}: {language}");
            }
        }
    }
}

/// Checks that a FIFO is rejected without waiting for its write end to be opened.
#[cfg(unix)]
pub(super) fn assert_fifo_rejected(read: fn(&Utf8Path) -> Result<(), Message>) {
    assert_fifo_operation_rejected(read, None, "files_io_not_a_file");
}

/// Checks that saving below a FIFO reports a non-directory parent without blocking.
#[cfg(unix)]
pub(super) fn assert_fifo_parent_rejected(write: fn(&Utf8Path) -> Result<(), Message>) {
    assert_fifo_operation_rejected(write, Some("document.json"), "files_io_not_a_directory");
}

/// Runs a FIFO operation in an isolated subprocess with a timeout.
#[cfg(unix)]
fn assert_fifo_operation_rejected(
    operation: fn(&Utf8Path) -> Result<(), Message>,
    file_name: Option<&str>,
    expected_key: &str,
) {
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};

    /// Passes the operation's path to the isolated subprocess.
    const FIFO_PATH_ENV: &str = "HRAFNIX_FILES_TEST_FIFO_PATH";

    if let Some(path) = std::env::var_os(FIFO_PATH_ENV) {
        let path = Utf8PathBuf::from_path_buf(path.into()).unwrap();
        let error = operation(&path).unwrap_err();
        assert_eq!(error.translate_data().message_key().as_str(), expected_key);
        assert_eq!(
            error
                .translate_data()
                .message_params()
                .get("path")
                .map(hrafnix_shareable_string::ShareableString::as_str),
            Some(path.as_str())
        );
        return;
    }

    let directory = TestDirectory::new();
    let path = directory.path.join("fifo");
    assert!(
        Command::new("mkfifo")
            .arg(&path)
            .status()
            .unwrap()
            .success()
    );
    let path = match file_name {
        Some(name) => path.join(name),
        None => path,
    };
    // Isolate a potentially blocking regression without needing nonblocking OS flags.
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            std::thread::current().name().unwrap(),
            "--nocapture",
        ])
        .env(FIFO_PATH_ENV, &path)
        .stdout(Stdio::null())
        .spawn()
        .unwrap();
    let started = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if started.elapsed() >= Duration::from_secs(2) {
            child.kill().unwrap();
            break child.wait().unwrap();
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    assert!(
        status.success(),
        "FIFO operation failed or timed out: {status}"
    );
}

/// Checks that an existing read-only file is neither replaced nor accompanied by a
/// leftover temporary file.
pub(super) fn assert_read_only_file_preserved(write: fn(&Utf8Path) -> Result<(), Message>) {
    let directory = TestDirectory::new();
    let path = directory.path.join("read-only.json");
    fs::write(&path, "original contents").unwrap();
    let mut permissions = fs::metadata(&path).unwrap().permissions();
    permissions.set_readonly(true);
    fs::set_permissions(&path, permissions).unwrap();

    let error = write(&path).unwrap_err();
    let still_read_only = fs::metadata(&path).unwrap().permissions().readonly();
    // Windows refuses to delete read-only files when the directory is cleaned up.
    #[cfg(windows)]
    #[allow(
        clippy::permissions_set_readonly_false,
        reason = "Clearing the attribute is intended and only happens on Windows."
    )]
    {
        let mut permissions = fs::metadata(&path).unwrap().permissions();
        permissions.set_readonly(false);
        fs::set_permissions(&path, permissions).unwrap();
    }

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_permission_denied"
    );
    assert_eq!(
        error
            .translate_data()
            .message_params()
            .get("path")
            .unwrap()
            .as_str(),
        path.as_str()
    );
    assert!(
        error
            .extra_detail()
            .is_some_and(|detail| detail.as_str().contains("read-only"))
    );
    assert!(still_read_only);
    assert_eq!(fs::read_to_string(&path).unwrap(), "original contents");
    assert_eq!(directory.entries(), ["read-only.json"]);
}

/// Checks that a file just over [`MAX_FILE_SIZE`] is rejected without being read.
///
/// The file is extended with `set_len`, which is sparse on common file systems, so the
/// test neither writes nor reads 256 MiB.
pub(super) fn assert_oversized_file_rejected(read: fn(&Utf8Path) -> Result<(), Message>) {
    let directory = TestDirectory::new();
    let path = directory.path.join("oversized.json");
    fs::File::create(&path)
        .unwrap()
        .set_len(MAX_FILE_SIZE.checked_add(1).unwrap())
        .unwrap();

    let error = read(&path).unwrap_err();

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_file_too_large"
    );
    assert_eq!(
        error
            .translate_data()
            .message_params()
            .get("path")
            .unwrap()
            .as_str(),
        path.as_str()
    );
    assert!(
        error
            .extra_detail()
            .is_some_and(|detail| detail.as_str().contains("268435456-byte"))
    );
}

/// Checks that a directory-open failure occurs before replacing an existing file.
#[cfg(unix)]
pub(super) fn assert_save_error_preserves_file(write: fn(&Utf8Path) -> Result<(), Message>) {
    use std::os::unix::fs::PermissionsExt;

    let directory = TestDirectory::new();
    let path = directory.path.join("existing.json");
    fs::write(&path, "original contents").unwrap();
    let permissions = fs::metadata(&directory.path).unwrap().permissions();
    fs::set_permissions(&directory.path, fs::Permissions::from_mode(0o300)).unwrap();
    let can_read_directory = fs::File::open(&directory.path).is_ok();
    let result = write(&path);
    fs::set_permissions(&directory.path, permissions).unwrap();
    if can_read_directory {
        eprintln!(
            "Skipping permission assertions: this process can read a directory with mode 0300"
        );
        return;
    }
    let error = result.unwrap_err();
    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_permission_denied"
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), "original contents");
    assert_eq!(directory.entries(), ["existing.json"]);
    assert_eq!(directory.sibling_temporary_files(), Vec::<String>::new());
}
