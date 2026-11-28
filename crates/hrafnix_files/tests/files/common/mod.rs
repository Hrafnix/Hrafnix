//! Existing regression helpers for public file API integration tests.

use camino::{Utf8Path, Utf8PathBuf};
use hrafnix_files::data::component::ComponentFile;
use hrafnix_message::message::Message;
use serde_json::{Value, json};
use std::fs;
use std::io::ErrorKind;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Maximum supported file size, in bytes (256 MiB).
const MAX_FILE_SIZE: u64 = 256 * 1024 * 1024;

/// Compares persisted component content while allowing runtime read provenance to differ.
pub(super) fn assert_component_content_eq_ref(expected: &ComponentFile, actual: &ComponentFile) {
    assert_eq!(actual.metadata(), expected.metadata());
    assert_eq!(actual.gui_metadata(), expected.gui_metadata());
    assert_eq!(
        actual.foreground_decorations(),
        expected.foreground_decorations()
    );
    assert_eq!(
        actual.background_decorations(),
        expected.background_decorations()
    );
    assert_eq!(actual.components(), expected.components());
    assert_eq!(actual.connections(), expected.connections());
    assert_eq!(actual.translations(), expected.translations());
    assert_eq!(
        actual.parameter_definition(),
        expected.parameter_definition()
    );
    assert_eq!(actual.variable_definition(), expected.variable_definition());
}

/// Valid version 1 metadata entry.
pub(super) const METADATA: &str = "{\"display-name\": \"\", \"description\": \"\", \"author\": \"\", \"minimum-version-major\": 0, \"minimum-version-minor\": 0}";

/// Valid version 1 translations entry with an explicit empty table.
pub(super) const TRANSLATIONS: &str = "{\"language\": \"en\", \"translations\": {}}";

/// Required entries specific to components or models.
fn root_specific_entries(root: &str) -> &'static str {
    if root == "component" {
        ", \"gui_metadata\": {\"dimensions\": {\"width\": 100, \"height\": 50}, \"rotation\": 0, \"icon\": \"../icons/component.svg\", \"ports\": []}"
    } else {
        ", \"global_settings\": []"
    }
}

/// Valid version 1 contents of a model or component root entry.
pub(super) fn contents(root: &str) -> String {
    let root_entries = root_specific_entries(root);
    format!(
        "{{\"metadata\": {METADATA}, \"translations\": {TRANSLATIONS}{root_entries}, \"foreground_decorations\": [], \"background_decorations\": [], \"components\": [], \"connections\": [], \"parameter_definition\": [], \"variable_definition\": []}}"
    )
}

/// Returns the expected JSON representation of the example connections.
pub(super) fn connections_json() -> Value {
    json!([
        {
            "from_component": "simulation_clock",
            "from_port": "time",
            "to_component": "heater",
            "to_port": "temperature_input",
            "line": {"midpoints": [{"x": 210, "y": 25}]},
        },
        {
            "from_component": "temperature_sensor",
            "from_port": "temperature",
            "to_component": "heater",
            "to_port": "temperature_input",
            "line": {"midpoints": [{"x": 295, "y": 30}]},
        },
    ])
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

/// Checks supported formatting, field order, and escaped root names.
pub(super) fn assert_document_variants(root: &str, read: fn(&Utf8Path) -> Result<(), Message>) {
    let root_entries = root_specific_entries(root);
    let directory = TestDirectory::new();
    let path = directory.path.join("variants.json");
    let contents = format!(
        "{{\"metadata\":{{\"display-name\":\"Example\",\"description\":\"An example {root}.\",\"author\":\"Hrafnix\",\"minimum-version-major\":1,\"minimum-version-minor\":2}},\"translations\":{TRANSLATIONS}{root_entries},\"foreground_decorations\":[],\"background_decorations\":[],\"components\":[],\"connections\":[],\"parameter_definition\":[],\"variable_definition\":[]}}"
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
    let contents = contents(root);
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
            format!("\u{feff}\u{feff}{{\"file-version\":\"1\",\"{root}\":{contents}}}"),
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

/// Checks that envelope and nested schema errors retain file diagnostic context.
pub(super) fn assert_schema_error_context(root: &str, read: fn(&Utf8Path) -> Result<(), Message>) {
    let contents = contents(root);
    let directory = TestDirectory::new();
    let path = directory.path.join("schema.json");
    let invalid_documents = [
        "[]".to_owned(),
        format!("{{\"{root}\":{contents}}}"),
        format!("{{\"file-version\":\"2\",\"{root}\":{contents}}}"),
        format!("{{\"file-version\":\"1\",\"{root}\":{contents},\"unknown\":{{}}}}"),
        format!("{{\"file-version\":\"1\",\"file-version\":\"1\",\"{root}\":{contents}}}"),
        format!(
            "{{\"file-version\":\"1\",\"{root}\":{}}}",
            contents.replace("\"display-name\": \"\"", "\"display-name\": 1"),
        ),
    ];
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
    let contents = contents(root);
    let directory = TestDirectory::new();
    let path = directory.path.join("trailing.json");
    for document in [
        format!("{{\"file-version\": \"1\", \"{root}\": {contents}}}"),
        format!("{{\n  \"file-version\": \"1\",\n  \"{root}\": {contents}\n}}"),
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
    let contents = contents(root);
    let document = format!("{{\"file-version\": \"1\", \"{root}\": {contents}}}");

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
