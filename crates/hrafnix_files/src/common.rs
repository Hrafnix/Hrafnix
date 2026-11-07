use std::collections::HashMap;
use std::{fs, io};
use std::fs::File;
use std::io::{ErrorKind, Read};
use camino::Utf8Path;
use hrafnix_message::message::{Message, MessageCategory, MessageLevel};

/// Maximum file size for reads and writes, in bytes (256 MiB).
pub const MAX_FILE_SIZE: u64 = 256 * 1024 * 1024;

/// Reads a regular file as UTF-8, rejecting files larger than [`MAX_FILE_SIZE`].
///
/// Checks the path before opening to reject FIFOs, then validates the opened
/// handle. A FIFO substituted after the path check can still block during opening.
///
/// Reports invalid UTF-8 by byte position, separately from I/O errors.
pub(crate) fn read_file_to_string(path: &Utf8Path) -> Result<String, Message> {
    let metadata = fs::metadata(path).map_err(|error| file_io_error(path, &error))?;
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

    String::from_utf8(bytes).map_err(|error| {
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
    })
}

/// Creates a localized I/O error with the OS diagnostic as extra detail.
pub(crate) fn file_io_error(path: &Utf8Path, error: &io::Error) -> Message {
    #[allow(
        clippy::wildcard_enum_match_arm,
        reason = "The remaining known variants are unstable and cannot be named on stable Rust."
    )]
    let key = match error.kind() {
        ErrorKind::NotFound => "files_io_not_found",
        ErrorKind::PermissionDenied => "files_io_permission_denied",
        ErrorKind::ConnectionRefused => "files_io_connection_refused",
        ErrorKind::ConnectionReset => "files_io_connection_reset",
        ErrorKind::HostUnreachable => "files_io_host_unreachable",
        ErrorKind::NetworkUnreachable => "files_io_network_unreachable",
        ErrorKind::ConnectionAborted => "files_io_connection_aborted",
        ErrorKind::NotConnected => "files_io_not_connected",
        ErrorKind::AddrInUse => "files_io_address_in_use",
        ErrorKind::AddrNotAvailable => "files_io_address_not_available",
        ErrorKind::NetworkDown => "files_io_network_down",
        ErrorKind::BrokenPipe => "files_io_broken_pipe",
        ErrorKind::AlreadyExists => "files_io_already_exists",
        ErrorKind::WouldBlock => "files_io_would_block",
        ErrorKind::NotADirectory => "files_io_not_a_directory",
        ErrorKind::IsADirectory => "files_io_not_a_file",
        ErrorKind::DirectoryNotEmpty => "files_io_directory_not_empty",
        ErrorKind::ReadOnlyFilesystem => "files_io_read_only_filesystem",
        ErrorKind::StaleNetworkFileHandle => "files_io_stale_network_file_handle",
        ErrorKind::InvalidInput => "files_io_invalid_input",
        ErrorKind::InvalidData => "files_io_invalid_data",
        ErrorKind::TimedOut => "files_io_timed_out",
        ErrorKind::WriteZero => "files_io_write_zero",
        ErrorKind::StorageFull => "files_io_storage_full",
        ErrorKind::NotSeekable => "files_io_not_seekable",
        ErrorKind::QuotaExceeded => "files_io_quota_exceeded",
        ErrorKind::FileTooLarge => "files_io_file_too_large",
        ErrorKind::ResourceBusy => "files_io_resource_busy",
        ErrorKind::ExecutableFileBusy => "files_io_executable_file_busy",
        ErrorKind::Deadlock => "files_io_deadlock",
        ErrorKind::CrossesDevices => "files_io_crosses_devices",
        ErrorKind::TooManyLinks => "files_io_too_many_links",
        ErrorKind::InvalidFilename => "files_io_invalid_filename",
        ErrorKind::ArgumentListTooLong => "files_io_argument_list_too_long",
        ErrorKind::Interrupted => "files_io_interrupted",
        ErrorKind::Unsupported => "files_io_unsupported",
        ErrorKind::UnexpectedEof => "files_io_unexpected_eof",
        ErrorKind::OutOfMemory => "files_io_out_of_memory",
        ErrorKind::Other => "files_io_other",
        _ => "files_io_error",
    };

    Message::new_with_params(
        MessageLevel::Error,
        MessageCategory::File,
        key.into(),
        HashMap::from([("path".into(), path.as_str().into())]),
        Some(error.to_string().into()),
    )
}