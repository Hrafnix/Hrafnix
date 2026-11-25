//! Tests for strict project manifests and atomic creation.

use super::common::{self, TestDirectory};
use hrafnix_files::builder::project_file;
use hrafnix_files::formats::project::{create_project_file, read_project_file, write_project_file};
use hrafnix_message::message::{MessageCategory, MessageLevel};
use hrafnix_shareable_string::SharedStringStore;
use hrafnix_translation::generate_translation_map;
use std::fs;
use std::sync::Barrier;

#[test]
fn project_manifests_round_trip_strings_and_launder_metadata() {
    let directory = TestDirectory::new();
    let path = directory.path.join("hrafnix.toml");
    for (name, version) in [
        ("Demo", "1.2.3"),
        ("", ""),
        ("Quoted \"name\"\n\u{e9}\u{754c}", "development\\branch"),
    ] {
        let expected = project_file(name.into(), version.into());
        write_project_file(&expected, &path).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert!(text.starts_with("[project]\n"));
        assert!(text.ends_with('\n'));
        let mut actual = read_project_file(&path).unwrap();
        assert_eq!(actual, expected);

        let store = SharedStringStore::new();
        actual.launder(&store);
        assert_eq!(actual, expected);
        assert!(std::ptr::eq(
            actual.name().as_str().as_ptr(),
            store.get(name).as_str().as_ptr(),
        ));
        assert!(std::ptr::eq(
            actual.version().as_str().as_ptr(),
            store.get(version).as_str().as_ptr(),
        ));
        assert_eq!(directory.entries(), ["hrafnix.toml"]);
    }
}

#[test]
fn project_reader_accepts_bom_comments_and_inline_tables() {
    let directory = TestDirectory::new();
    let path = directory.path.join("hrafnix.toml");
    let expected = project_file("Demo".into(), "1.2.3".into());
    for text in [
        "\u{feff}# comment\r\n[project]\r\nversion = '1.2.3'\r\nname = 'Demo'\r\n",
        "project = { version = '1.2.3', name = 'Demo' }\n",
    ] {
        fs::write(&path, text).unwrap();
        assert_eq!(read_project_file(&path).unwrap(), expected);
    }
}

#[test]
fn project_reader_rejects_missing_unknown_duplicate_and_wrongly_typed_fields() {
    let directory = TestDirectory::new();
    let path = directory.path.join("hrafnix.toml");
    let map = generate_translation_map(&SharedStringStore::new());
    for text in [
        "",
        "name = 'Demo'\nversion = '1.2.3'\n",
        "[project]\nname = 'Demo'\n",
        "[project]\nversion = '1.2.3'\n",
        "extra = true\n[project]\nname = 'Demo'\nversion = '1.2.3'\n",
        "[project]\nname = 'Demo'\nversion = '1.2.3'\nextra = true\n",
        "[project]\nname = 'Demo'\nname = 'Duplicate'\nversion = '1.2.3'\n",
        "[project]\nname = 'Demo'\nversion = '1.2.3'\nversion = '2'\n",
        "[project]\nname = 123\nversion = '1.2.3'\n",
        "[project]\nname = 'Demo'\nversion = 1\n",
        "project = ['Demo', '1.2.3']\n",
        "[[project]]\nname = 'Demo'\nversion = '1.2.3'\n",
        "[project]\nname = 'unterminated\nversion = '1.2.3'\n",
    ] {
        fs::write(&path, text).unwrap();
        let error = read_project_file(&path).unwrap_err();
        assert_eq!(error.level(), MessageLevel::Error, "{text}");
        assert_eq!(error.category(), MessageCategory::File, "{text}");
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_toml_data",
            "{text}",
        );
        assert_eq!(
            error
                .translate_data()
                .message_params()
                .get("path")
                .unwrap()
                .as_str(),
            path.as_str(),
        );
        assert!(error.extra_detail().is_some(), "{text}");
        for language in ["en", "zh", "de", "es", "fr", "ja", "ko"] {
            let translated = error.translated_message(&map, language).unwrap();
            assert!(translated.as_str().contains(path.as_str()));
            assert!(!translated.as_str().contains("%{"));
        }
    }
}

#[test]
fn project_reader_reports_io_and_utf8_errors_separately() {
    let directory = TestDirectory::new();
    let path = directory.path.join("hrafnix.toml");
    let missing = read_project_file(&path).unwrap_err();
    assert_eq!(
        missing.translate_data().message_key().as_str(),
        "files_io_not_found"
    );
    fs::write(&path, b"\xFF").unwrap();
    let invalid = read_project_file(&path).unwrap_err();
    assert_eq!(
        invalid.translate_data().message_key().as_str(),
        "files_invalid_utf8"
    );
    let not_file = read_project_file(&directory.path).unwrap_err();
    assert_eq!(
        not_file.translate_data().message_key().as_str(),
        "files_io_not_a_file"
    );
}

#[test]
fn project_creation_never_replaces_existing_paths_and_cleans_temporary_files() {
    let directory = TestDirectory::new();
    let path = directory.path.join("hrafnix.toml");
    let original = project_file("Original".into(), "1".into());
    create_project_file(&original, &path).unwrap();
    let original_bytes = fs::read(&path).unwrap();
    let error =
        create_project_file(&project_file("Replacement".into(), "2".into()), &path).unwrap_err();
    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_already_exists"
    );
    assert_eq!(fs::read(&path).unwrap(), original_bytes);
    assert_eq!(read_project_file(&path).unwrap(), original);
    assert_eq!(directory.entries(), ["hrafnix.toml"]);

    fs::remove_file(&path).unwrap();
    fs::create_dir(&path).unwrap();
    assert!(create_project_file(&project_file("Demo".into(), "1".into()), &path).is_err());
    assert!(path.is_dir());
    assert_eq!(directory.entries(), ["hrafnix.toml"]);
}

#[test]
fn concurrent_project_creators_publish_exactly_one_complete_manifest() {
    let directory = TestDirectory::new();
    let path = directory.path.join("hrafnix.toml");
    let names = ["first", "second", "third", "fourth"];
    let barrier = Barrier::new(names.len());
    let successes = std::thread::scope(|scope| {
        let handles: Vec<_> = names
            .iter()
            .map(|name| {
                let path = &path;
                let barrier = &barrier;
                scope.spawn(move || {
                    barrier.wait();
                    create_project_file(&project_file((*name).into(), "1".into()), path)
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| match handle.join().unwrap() {
                Ok(()) => 1,
                Err(error) => {
                    assert_eq!(
                        error.translate_data().message_key().as_str(),
                        "files_io_already_exists"
                    );
                    0
                }
            })
            .sum::<usize>()
    });
    assert_eq!(successes, 1);
    let loaded = read_project_file(&path).unwrap();
    assert!(names.contains(&loaded.name().as_str()));
    assert_eq!(loaded.version().as_str(), "1");
    assert_eq!(directory.entries(), ["hrafnix.toml"]);
}

#[cfg(unix)]
#[test]
fn project_creation_preserves_dangling_symlinks() {
    let directory = TestDirectory::new();
    let path = directory.path.join("hrafnix.toml");
    let target = directory.path.join("missing");
    std::os::unix::fs::symlink(&target, &path).unwrap();
    let error = create_project_file(&project_file("Demo".into(), "1".into()), &path).unwrap_err();
    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_already_exists"
    );
    assert_eq!(fs::read_link(&path).unwrap(), target.as_std_path());
    assert!(!target.exists());
    assert_eq!(directory.entries(), ["hrafnix.toml"]);
}

#[test]
fn oversized_project_manifests_are_rejected_before_parsing() {
    common::assert_oversized_file_rejected(|path| read_project_file(path).map(|_| ()));
}

#[test]
fn read_only_project_manifests_are_not_replaced() {
    common::assert_read_only_file_preserved(|path| {
        write_project_file(&project_file("Demo".into(), "1".into()), path)
    });
}
