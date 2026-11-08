use crate::common::error::file_io_error;
use camino::{Utf8Path, Utf8PathBuf};
use hrafnix_message::message::{Message, MessageCategory, MessageLevel};
use std::collections::HashMap;
use std::fs::{File, OpenOptions, metadata};
use std::io::{ErrorKind, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io};

/// Maximum file size for reads and writes, in bytes (256 MiB).
const MAX_FILE_SIZE: u64 = 256 * 1024 * 1024;

/// Reads a regular file as UTF-8, rejecting files larger than [`MAX_FILE_SIZE`].
///
/// Checks the path before opening to reject FIFOs, then validates the opened
/// handle. A FIFO substituted after the path check can still block during opening.
///
/// Removes a single leading UTF-8 BOM after decoding. Reports invalid UTF-8 by
/// its original byte position, separately from I/O errors.
pub(crate) fn read_file_to_string(path: &Utf8Path) -> Result<String, Message> {
    let metadata = metadata(path).map_err(|error| file_io_error(path, &error))?;
    if !metadata.is_file() {
        return Err(Message::new_with_params(
            MessageLevel::Error,
            MessageCategory::File,
            "files_io_not_a_file".into(),
            HashMap::from([("path".into(), path.as_str().into())]),
            None,
        ));
    }

    let file = File::open(path).map_err(|error| file_io_error(path, &error))?;

    // Validate the opened file in case the path changed.
    let metadata = file
        .metadata()
        .map_err(|error| file_io_error(path, &error))?;
    if !metadata.is_file() {
        return Err(Message::new_with_params(
            MessageLevel::Error,
            MessageCategory::File,
            "files_io_not_a_file".into(),
            HashMap::from([("path".into(), path.as_str().into())]),
            None,
        ));
    }

    if metadata.len() > MAX_FILE_SIZE {
        return Err(Message::new_with_params(
            MessageLevel::Error,
            MessageCategory::File,
            "files_io_file_too_large".into(),
            HashMap::from([("path".into(), path.as_str().into())]),
            Some(format!("file exceeds the {MAX_FILE_SIZE}-byte size limit").into()),
        ));
    }

    let mut bytes = Vec::with_capacity(usize::try_from(metadata.len()).unwrap_or(0));
    file.take(MAX_FILE_SIZE.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| file_io_error(path, &error))?;
    if u64::try_from(bytes.len()).map_or(true, |read| read > MAX_FILE_SIZE) {
        return Err(Message::new_with_params(
            MessageLevel::Error,
            MessageCategory::File,
            "files_io_file_too_large".into(),
            HashMap::from([("path".into(), path.as_str().into())]),
            Some(format!("file exceeds the {MAX_FILE_SIZE}-byte size limit").into()),
        ));
    }

    let mut contents = String::from_utf8(bytes).map_err(|error| {
        let error = error.utf8_error();
        let mut params = HashMap::from([
            ("path".into(), path.as_str().into()),
            ("position".into(), error.valid_up_to().to_string().into()),
        ]);
        let key = match error.error_len() {
            Some(length) => {
                params.insert("length".into(), length.to_string().into());
                "files_invalid_utf8"
            }
            None => "files_incomplete_utf8",
        };

        Message::new_with_params(
            MessageLevel::Error,
            MessageCategory::File,
            key.into(),
            params,
            Some(error.to_string().into()),
        )
    })?;

    if contents.starts_with('\u{feff}') {
        contents.remove(0);
    }
    Ok(contents)
}

/// Atomically replaces `path` with the exact UTF-8 bytes of `contents`.
///
/// No serialization, formatting, or final newline is added.
///
/// Contents larger than [`MAX_FILE_SIZE`] are rejected before touching the
/// destination. A uniquely named temporary file is written and synced beside the
/// destination before being renamed over it. Failures before the rename leave an
/// existing destination untouched; a later parent-directory sync failure reports
/// that replacement succeeded but crash durability is uncertain.
///
/// Read-only regular files, including symbolic link targets, are not replaced.
/// This check is not atomic with the rename. Symbolic links themselves are replaced,
/// leaving their targets untouched. Ownership and extended attributes are not
/// preserved. On Unix, regular-file permission bits are copied, excluding
/// set-user-ID, set-group-ID, and sticky bits.
///
/// File systems that cannot sync are treated as best-effort. All errors report
/// the destination path. Parent-directory checks precede opening, but concurrent
/// replacement with a special file can still block the open.
pub(crate) fn write_string_to_file(contents: &str, path: &Utf8Path) -> Result<(), Message> {
    if u64::try_from(contents.len()).map_or(true, |length| length > MAX_FILE_SIZE) {
        return Err(Message::new_with_params(
            MessageLevel::Error,
            MessageCategory::File,
            "files_io_file_too_large".into(),
            HashMap::from([("path".into(), path.as_str().into())]),
            Some(format!("file exceeds the {MAX_FILE_SIZE}-byte size limit").into()),
        ));
    }

    reject_read_only_destination(path)?;

    #[cfg(unix)]
    let parent_directory =
        open_parent_directory(path).map_err(|error| file_io_error(path, &error))?;
    #[cfg(not(unix))]
    check_parent_directory(path).map_err(|error| file_io_error(path, &error))?;

    let (temporary_path, file) = create_temporary_file(path)?;
    if let Err(error) = write_and_replace(file, &temporary_path, path, contents) {
        // The file handle has been closed, including when writing or syncing failed.
        if let Err(cleanup_error) = fs::remove_file(&temporary_path) {
            return Err(file_io_error(
                path,
                &io::Error::new(
                    error.kind(),
                    format!("{error}; temporary file cleanup failed: {cleanup_error}"),
                ),
            ));
        }
        return Err(file_io_error(path, &error));
    }

    #[cfg(unix)]
    sync_parent_directory(path, &parent_directory)?;

    Ok(())
}

/// Refuses to replace an existing regular file or symbolic link target marked read-only.
///
/// Directory write access otherwise allows replacing a read-only file on Unix.
/// Missing destinations and non-regular files are left for later steps.
fn reject_read_only_destination(path: &Utf8Path) -> Result<(), Message> {
    if let Ok(metadata) = metadata(path)
        && metadata.is_file()
        && metadata.permissions().readonly()
    {
        return Err(Message::new_with_params(
            MessageLevel::Error,
            MessageCategory::File,
            "files_io_permission_denied".into(),
            HashMap::from([("path".into(), path.as_str().into())]),
            Some("destination file is read-only".into()),
        ));
    }

    Ok(())
}

/// Creates a hidden temporary file beside `path`, retrying colliding names.
///
/// Names use the process ID, a timestamp, and a local retry counter. Creation is
/// exclusive so a collision cannot overwrite an existing file.
fn create_temporary_file(path: &Utf8Path) -> Result<(Utf8PathBuf, File), Message> {
    const TEMPORARY_FILE_ATTEMPTS: u32 = 100;

    let Some(file_name) = path.file_name() else {
        return Err(file_io_error(
            path,
            &io::Error::new(ErrorKind::InvalidInput, "path has no file name"),
        ));
    };
    // Limit the prefix to 128 UTF-8 bytes, leaving space for the unique suffix.
    let prefix: String = file_name.chars().take(32).collect();
    let process = std::process::id();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| file_io_error(path, &io::Error::other(error)))?
        .as_nanos();
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        options.mode(temporary_file_mode(path));
    }

    for attempt in 0..TEMPORARY_FILE_ATTEMPTS {
        let temporary_path =
            path.with_file_name(format!(".{prefix}.{process}-{timestamp}-{attempt}.tmp"));
        match options.open(&temporary_path) {
            Ok(file) => return Ok((temporary_path, file)),
            Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
            Err(error) => return Err(file_io_error(path, &error)),
        }
    }

    Err(Message::new_with_params(
        MessageLevel::Error,
        MessageCategory::File,
        "files_io_error".into(),
        HashMap::from([("path".into(), path.as_str().into())]),
        Some(
            format!(
                "could not create a unique temporary file after {TEMPORARY_FILE_ATTEMPTS} attempts"
            )
            .into(),
        ),
    ))
}

/// Keeps replacement temporary files owner-only until permissions are copied.
///
/// Only definitely missing destinations use the normal creation mode, less the umask.
#[cfg(unix)]
fn temporary_file_mode(path: &Utf8Path) -> u32 {
    match metadata(path) {
        Err(error) if error.kind() == ErrorKind::NotFound => 0o666,
        _ => 0o600,
    }
}

/// Writes and syncs `contents`, then closes the file and renames it over `path`.
///
/// On Unix, regular-file permission bits are copied before writing, without special bits.
fn write_and_replace(
    mut file: File,
    temporary_path: &Utf8Path,
    path: &Utf8Path,
    contents: &str,
) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        if let Ok(metadata) = metadata(path)
            && metadata.is_file()
        {
            file.set_permissions(fs::Permissions::from_mode(
                metadata.permissions().mode() & 0o777,
            ))?;
        }
    }

    file.write_all(contents.as_bytes())?;
    sync_best_effort(&file)?;
    drop(file);
    fs::rename(temporary_path, path)
}

/// Syncs a file or directory, treating unsupported syncing as best-effort.
fn sync_best_effort(file: &File) -> io::Result<()> {
    match file.sync_all() {
        Err(error) if is_sync_unsupported(&error) => Ok(()),
        result => result,
    }
}

/// Distinguishes unsupported syncing from a genuine sync failure.
fn is_sync_unsupported(error: &io::Error) -> bool {
    // Apple's F_FULLFSYNC may return ENOTSUP on network, exFAT, or FUSE volumes.
    #[cfg(target_vendor = "apple")]
    {
        const ENOTSUP: i32 = 45;
        if error.raw_os_error() == Some(ENOTSUP) {
            return true;
        }
    }
    error.kind() == ErrorKind::Unsupported
}

/// Returns the existing parent directory, reporting a non-directory parent consistently.
///
/// Without this check, Windows reports a regular-file parent as not found.
fn check_parent_directory(path: &Utf8Path) -> io::Result<&Utf8Path> {
    let parent = match path.parent() {
        Some(parent) if !parent.as_str().is_empty() => parent,
        _ => Utf8Path::new("."),
    };
    if !metadata(parent)?.is_dir() {
        return Err(ErrorKind::NotADirectory.into());
    }
    Ok(parent)
}

/// Opens the parent before replacement so access errors leave the destination untouched.
///
/// Metadata checks precede opening, but concurrent replacement with a special file
/// can still block the open.
#[cfg(unix)]
fn open_parent_directory(path: &Utf8Path) -> io::Result<File> {
    let parent = check_parent_directory(path)?;
    let directory = File::open(parent)?;
    if !directory.metadata()?.is_dir() {
        return Err(ErrorKind::NotADirectory.into());
    }
    Ok(directory)
}

/// Reports directory-sync failures separately because replacement has already succeeded.
#[cfg(unix)]
fn sync_parent_directory(path: &Utf8Path, directory: &File) -> Result<(), Message> {
    sync_best_effort(directory).map_err(|error| {
        Message::new_with_params(
            MessageLevel::Error,
            MessageCategory::File,
            "files_io_replaced_sync_failed".into(),
            HashMap::from([("path".into(), path.as_str().into())]),
            Some(error.to_string().into()),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use camino::Utf8PathBuf;

    #[derive(Debug)]
    struct TestDirectory {
        path: Utf8PathBuf,
    }

    impl TestDirectory {
        fn new() -> Self {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();

            for attempt in 0..100 {
                let path = Utf8PathBuf::try_from(std::env::temp_dir())
                    .unwrap()
                    .join(format!(
                        "hrafnix-string-write-test-{}-{timestamp}-{attempt}",
                        std::process::id()
                    ));
                match fs::create_dir(&path) {
                    Ok(()) => return Self { path },
                    Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
                    Err(error) => panic!("Failed to create test directory '{path}': {error}"),
                }
            }
            panic!("Failed to create a unique test directory");
        }

        fn entries(&self) -> Vec<String> {
            let mut entries: Vec<String> = fs::read_dir(&self.path)
                .unwrap()
                .map(|entry| entry.unwrap().file_name().into_string().unwrap())
                .collect();
            entries.sort();
            entries
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            // Avoid a second panic when a test is already unwinding.
            drop(fs::remove_dir_all(&self.path));
        }
    }

    fn assert_error_path(error: &Message, path: &Utf8Path) {
        assert_eq!(
            error
                .translate_data()
                .message_params()
                .get("path")
                .unwrap()
                .as_str(),
            path.as_str()
        );
    }

    #[test]
    fn reads_utf8_and_removes_only_a_leading_bom() {
        let directory = TestDirectory::new();
        let path = directory.path.join("document.txt");

        for (contents, expected) in [
            ("", ""),
            ("plain \u{e9}\u{754c}", "plain \u{e9}\u{754c}"),
            ("\u{feff}", ""),
            ("\u{feff}text", "text"),
            ("\u{feff}\u{feff}text", "\u{feff}text"),
            ("text\u{feff}", "text\u{feff}"),
        ] {
            fs::write(&path, contents).unwrap();
            assert_eq!(read_file_to_string(&path).unwrap(), expected);
        }
    }

    #[test]
    fn utf8_errors_after_a_bom_keep_original_byte_positions() {
        let directory = TestDirectory::new();
        let path = directory.path.join("document.txt");

        for (suffix, key) in [
            (b"\xFF".as_slice(), "files_invalid_utf8"),
            (b"\xE2\x82".as_slice(), "files_incomplete_utf8"),
        ] {
            let bytes = ["\u{feff}text".as_bytes(), suffix].concat();
            fs::write(&path, bytes).unwrap();
            let error = read_file_to_string(&path).unwrap_err();
            assert_eq!(error.translate_data().message_key().as_str(), key);
            assert_eq!(
                error
                    .translate_data()
                    .message_params()
                    .get("position")
                    .unwrap()
                    .as_str(),
                "7"
            );
            assert_error_path(&error, &path);
        }
    }

    #[test]
    fn writes_exact_contents_and_replaces_existing_files() {
        let directory = TestDirectory::new();
        let path = directory.path.join("document.txt");

        for contents in ["not JSON: \u{e9}\u{754c}\0", "with\nnewlines\n", ""] {
            write_string_to_file(contents, &path).unwrap();
            assert_eq!(fs::read(&path).unwrap(), contents.as_bytes());
            assert_eq!(directory.entries(), ["document.txt"]);
        }
    }

    #[test]
    fn concurrent_writes_leave_one_complete_file_without_temporary_files() {
        let directory = TestDirectory::new();
        let path = directory.path.join("document.txt");
        let contents: Vec<String> = (0..16)
            .map(|id| format!("{id}:{}", "x".repeat(4096)))
            .collect();
        let barrier = std::sync::Barrier::new(contents.len());

        std::thread::scope(|scope| {
            let path = &path;
            let barrier = &barrier;
            let handles: Vec<_> = contents
                .iter()
                .map(|contents| {
                    scope.spawn(move || {
                        barrier.wait();
                        write_string_to_file(contents, path)
                    })
                })
                .collect();
            for handle in handles {
                handle.join().unwrap().unwrap();
            }
        });
        assert!(contents.contains(&fs::read_to_string(&path).unwrap()));
        assert_eq!(directory.entries(), ["document.txt"]);
    }

    #[test]
    fn preserves_read_only_destination() {
        let directory = TestDirectory::new();
        let path = directory.path.join("document.txt");
        fs::write(&path, "original").unwrap();
        let mut permissions = fs::metadata(&path).unwrap().permissions();
        permissions.set_readonly(true);
        fs::set_permissions(&path, permissions).unwrap();

        let result = write_string_to_file("replacement", &path);
        let still_read_only = fs::metadata(&path).unwrap().permissions().readonly();
        #[cfg(windows)]
        {
            let mut permissions = fs::metadata(&path).unwrap().permissions();
            permissions.set_readonly(false);
            fs::set_permissions(&path, permissions).unwrap();
        }
        let error = result.unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_io_permission_denied"
        );
        assert_error_path(&error, &path);
        assert!(still_read_only);
        assert_eq!(fs::read_to_string(&path).unwrap(), "original");
        assert_eq!(directory.entries(), ["document.txt"]);
    }

    #[test]
    fn removes_temporary_file_when_rename_fails() {
        let directory = TestDirectory::new();
        let path = directory.path.join("destination");
        fs::create_dir(&path).unwrap();
        fs::write(path.join("original.txt"), "original").unwrap();

        let error = write_string_to_file("replacement", &path).unwrap_err();
        assert_error_path(&error, &path);
        assert_eq!(
            fs::read_to_string(path.join("original.txt")).unwrap(),
            "original"
        );
        assert_eq!(directory.entries(), ["destination"]);
    }

    #[test]
    fn reports_destination_path_for_invalid_parents() {
        let directory = TestDirectory::new();
        let missing = directory.path.join("missing/document.txt");
        let error = write_string_to_file("contents", &missing).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_io_not_found"
        );
        assert_error_path(&error, &missing);

        let parent = directory.path.join("regular-file");
        fs::write(&parent, "original").unwrap();
        let path = parent.join("document.txt");
        let error = write_string_to_file("contents", &path).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_io_not_a_directory"
        );
        assert_error_path(&error, &path);
        assert_eq!(fs::read_to_string(&parent).unwrap(), "original");
        assert_eq!(directory.entries(), ["regular-file"]);
    }

    #[test]
    fn enforces_exact_size_limit_before_touching_destination() {
        let directory = TestDirectory::new();
        let path = directory.path.join("document.txt");
        let limit = usize::try_from(MAX_FILE_SIZE).unwrap();
        let mut contents = "x".repeat(limit.checked_add(1).unwrap());
        contents.replace_range(..2, "\u{e9}");
        assert_eq!(contents.chars().count(), limit);

        let error = write_string_to_file(&contents, &path).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_io_file_too_large"
        );
        assert_error_path(&error, &path);
        assert_eq!(directory.entries(), Vec::<String>::new());

        fs::write(&path, "original").unwrap();
        let error = write_string_to_file(&contents, &path).unwrap_err();
        assert_error_path(&error, &path);
        assert_eq!(fs::read_to_string(&path).unwrap(), "original");
        assert_eq!(directory.entries(), ["document.txt"]);

        fs::OpenOptions::new()
            .write(true)
            .open(&path)
            .unwrap()
            .set_len(MAX_FILE_SIZE.checked_add(1).unwrap())
            .unwrap();
        let error = read_file_to_string(&path).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_io_file_too_large"
        );
        assert_error_path(&error, &path);

        contents.truncate(limit);
        write_string_to_file(&contents, &path).unwrap();
        assert_eq!(fs::metadata(&path).unwrap().len(), MAX_FILE_SIZE);
        assert_eq!(read_file_to_string(&path).unwrap(), contents);
        assert_eq!(directory.entries(), ["document.txt"]);
    }

    #[cfg(unix)]
    #[test]
    fn sync_failures_explicitly_report_that_replacement_has_committed() {
        use hrafnix_shareable_string::SharedStringStore;
        use hrafnix_translation::generate_translation_map;

        let path = Utf8Path::new("model.hxm");
        // A device cannot be fsynced, providing a deterministic sync failure.
        let error = sync_parent_directory(path, &File::open("/dev/null").unwrap()).unwrap_err();
        assert_eq!(error.level(), MessageLevel::Error);
        assert_eq!(error.category(), MessageCategory::File);
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_io_replaced_sync_failed"
        );
        assert_eq!(error.translate_data().message_params().len(), 1);
        assert_error_path(&error, path);
        assert_eq!(
            error.extra_detail().unwrap().as_str(),
            File::open("/dev/null")
                .unwrap()
                .sync_all()
                .unwrap_err()
                .to_string()
        );
        let map = generate_translation_map(&SharedStringStore::new());
        let english = error.translated_message(&map, "en").unwrap();
        assert_eq!(
            english.as_str(),
            "File 'model.hxm' has been replaced, but syncing its parent directory failed; persistence across a crash is uncertain"
        );
        for language in ["en", "zh", "de", "es", "fr", "ja", "ko"] {
            let translated = error.translated_message(&map, language).unwrap();
            assert!(translated.as_str().contains(path.as_str()));
            assert!(!translated.as_str().contains("%{"));
            if language != "en" {
                assert_ne!(translated, english, "{language}");
            }
        }
    }

    #[test]
    fn unsupported_sync_is_best_effort_but_other_sync_failures_are_not() {
        assert!(is_sync_unsupported(&io::Error::from(
            ErrorKind::Unsupported
        )));
        #[cfg(target_vendor = "apple")]
        assert!(is_sync_unsupported(&io::Error::from_raw_os_error(45)));

        for kind in [
            ErrorKind::PermissionDenied,
            ErrorKind::InvalidInput,
            ErrorKind::Other,
        ] {
            assert!(!is_sync_unsupported(&io::Error::from(kind)), "{kind:?}");
        }
        #[cfg(unix)]
        {
            let file = File::open("/dev/null").unwrap();
            let error = file.sync_all().unwrap_err();
            assert!(!is_sync_unsupported(&error), "{error}");
            assert_eq!(sync_best_effort(&file).unwrap_err().kind(), error.kind());
        }
    }

    #[cfg(unix)]
    #[test]
    fn temporary_file_is_private_until_permissions_are_copied() {
        use std::os::unix::fs::PermissionsExt;

        let directory = TestDirectory::new();
        let path = directory.path.join("document.txt");
        let mode_of = |file: &File| file.metadata().unwrap().permissions().mode() & 0o7777;

        let (missing_path, missing) = create_temporary_file(&path).unwrap();
        let default_mode = mode_of(&File::create(directory.path.join("default")).unwrap());
        assert_eq!(mode_of(&missing), default_mode);
        fs::write(&path, "original").unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
        let (replacing_path, replacing) = create_temporary_file(&path).unwrap();
        assert_eq!(mode_of(&replacing), 0o600);
        assert_ne!(missing_path, replacing_path);
        assert_eq!(fs::read_to_string(&path).unwrap(), "original");
    }

    #[cfg(unix)]
    #[test]
    fn preserves_regular_permissions_without_special_bits() {
        use std::os::unix::fs::PermissionsExt;

        let directory = TestDirectory::new();
        let path = directory.path.join("document.txt");
        for mode in [0o600, 0o640, 0o604, 0o7751] {
            fs::write(&path, "original").unwrap();
            fs::set_permissions(&path, fs::Permissions::from_mode(mode)).unwrap();

            write_string_to_file("replacement", &path).unwrap();
            assert_eq!(
                fs::metadata(&path).unwrap().permissions().mode() & 0o7777,
                mode & 0o777,
                "{mode:o}"
            );
            assert_eq!(fs::read_to_string(&path).unwrap(), "replacement");
            assert_eq!(directory.entries(), ["document.txt"]);
        }
    }

    #[cfg(unix)]
    #[test]
    fn replaces_symbolic_link_without_changing_target() {
        use std::os::unix::fs::{PermissionsExt, symlink};

        let directory = TestDirectory::new();
        let target = directory.path.join("target.txt");
        let path = directory.path.join("link.txt");
        fs::write(&target, "original").unwrap();
        fs::set_permissions(&target, fs::Permissions::from_mode(0o640)).unwrap();
        symlink(&target, &path).unwrap();

        write_string_to_file("replacement", &path).unwrap();
        assert!(
            !fs::symlink_metadata(&path)
                .unwrap()
                .file_type()
                .is_symlink()
        );
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o640
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), "replacement");
        assert_eq!(fs::read_to_string(&target).unwrap(), "original");
        assert_eq!(directory.entries(), ["link.txt", "target.txt"]);
    }

    #[cfg(unix)]
    #[test]
    fn handles_long_ascii_and_utf8_filenames() {
        let directory = TestDirectory::new();

        for name in ["a".repeat(255), "\u{754c}".repeat(85)] {
            let path = directory.path.join(&name);
            assert_eq!(name.len(), 255);
            write_string_to_file("original", &path).unwrap();
            write_string_to_file("replacement", &path).unwrap();
            assert_eq!(fs::read_to_string(&path).unwrap(), "replacement");
            fs::remove_file(&path).unwrap();
            assert_eq!(directory.entries(), Vec::<String>::new());
        }
    }
}
