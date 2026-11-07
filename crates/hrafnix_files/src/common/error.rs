use camino::Utf8Path;
use hrafnix_message::message::{Message, MessageCategory, MessageLevel};
use serde_json::error::Category;
use std::collections::HashMap;
use std::io::{self, ErrorKind};

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

/// Maps a JSON parse error to a localized diagnostic by category, retaining the
/// parser's own message with its position as extra detail.
///
/// `serde_json` does not expose its individual error codes, so its categories are the
/// finest stable distinction available. The match is deliberately exhaustive without
/// a wildcard arm: a `serde_json` upgrade that adds a category must fail to compile
/// here until it has its own diagnostic key and translations.
pub(crate) fn json_error(path: &Utf8Path, error: serde_json::Error) -> Message {
    let key = match error.classify() {
        Category::Io => return file_io_error(path, &io::Error::from(error)),
        Category::Syntax => "files_json_syntax",
        Category::Eof => "files_json_unexpected_eof",
        Category::Data => "files_json_data",
    };
    let params = HashMap::from([
        ("line".into(), error.line().to_string().into()),
        ("column".into(), error.column().to_string().into()),
        ("path".into(), path.as_str().into()),
    ]);

    Message::new_with_params(
        MessageLevel::Error,
        MessageCategory::File,
        key.into(),
        params,
        Some(error.to_string().into()),
    )
}

/// Maps a JSON serialization error to a localized diagnostic.
///
/// Serialization failures have no positions and are retained as extra detail.
/// I/O failures keep their localized I/O diagnostic.
pub(crate) fn json_serialization_error(path: &Utf8Path, error: serde_json::Error) -> Message {
    if error.classify() == Category::Io {
        return file_io_error(path, &io::Error::from(error));
    }

    let params = HashMap::from([("path".into(), path.as_str().into())]);

    Message::new_with_params(
        MessageLevel::Error,
        MessageCategory::File,
        "files_json_serialization".into(),
        params,
        Some(error.to_string().into()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_shareable_string::{ShareableString, SharedStringStore};
    use hrafnix_translation::generate_translation_map;
    use serde::{Serialize, Serializer};
    use serde_json::Value;

    /// Returns the named parameter of `message` as text.
    fn param<'a>(message: &'a Message, name: &str) -> Option<&'a str> {
        message
            .translate_data()
            .message_params()
            .get(name)
            .map(ShareableString::as_str)
    }

    /// Checks every supported language and substitution, including the English text.
    fn assert_translates_everywhere(message: &Message, english: &str) {
        let map = generate_translation_map(&SharedStringStore::new());
        let key = message.translate_data().message_key().as_str();
        assert_eq!(
            message
                .translated_message(&map, "en")
                .as_ref()
                .map(ShareableString::as_str),
            Some(english),
            "{key}"
        );
        for language in ["en", "zh", "de", "es", "fr", "ja", "ko"] {
            let translated = message.translated_message(&map, language).unwrap();
            assert!(!translated.as_str().contains("%{"), "{key}: {language}");
            for value in message.translate_data().message_params().values() {
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

    #[test]
    fn every_json_error_category_has_a_localized_diagnostic() {
        let path = Utf8Path::new("component.hxc");
        let cases = [
            (
                serde_json::from_str::<Value>("}").unwrap_err(),
                "files_json_syntax",
                "1",
                "1",
                "'component.hxc': Invalid JSON syntax at line 1, column 1",
            ),
            (
                serde_json::from_str::<Value>("{\n  \"a\": [1,").unwrap_err(),
                "files_json_unexpected_eof",
                "2",
                "10",
                "'component.hxc': Unexpected end of JSON data at line 2, column 10",
            ),
            (
                serde_json::from_str::<u32>("\"text\"").unwrap_err(),
                "files_json_data",
                "1",
                "6",
                "'component.hxc': JSON data does not match the expected structure at line 1, column 6",
            ),
        ];

        for (error, key, line, column, english) in cases {
            let diagnostic = error.to_string();
            let message = json_error(path, error);

            assert_eq!(message.level(), MessageLevel::Error);
            assert_eq!(message.category(), MessageCategory::File);
            assert_eq!(message.translate_data().message_key().as_str(), key);
            assert_eq!(message.translate_data().message_params().len(), 3);
            assert_eq!(param(&message, "path"), Some(path.as_str()));
            assert_eq!(param(&message, "line"), Some(line));
            assert_eq!(param(&message, "column"), Some(column));
            assert_eq!(
                message.extra_detail().map(ShareableString::as_str),
                Some(diagnostic.as_str())
            );
            assert_translates_everywhere(&message, english);
        }
    }

    #[test]
    fn json_io_errors_preserve_file_io_diagnostics() {
        let path = Utf8Path::new("component.hxc");
        let error = io::Error::new(ErrorKind::PermissionDenied, "read denied");
        let expected = file_io_error(path, &error);

        assert_eq!(json_error(path, serde_json::Error::io(error)), expected);
    }

    #[test]
    fn serialization_errors_have_a_localized_diagnostic() {
        /// A value that cannot be represented in JSON.
        #[derive(Debug)]
        struct Failing;

        impl Serialize for Failing {
            fn serialize<S: Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom("unrepresentable value"))
            }
        }

        let path = Utf8Path::new("model.hxm");
        let error = serde_json::to_string(&Failing).unwrap_err();
        let diagnostic = error.to_string();
        let message = json_serialization_error(path, error);

        assert_eq!(message.level(), MessageLevel::Error);
        assert_eq!(message.category(), MessageCategory::File);
        assert_eq!(
            message.translate_data().message_key().as_str(),
            "files_json_serialization"
        );
        assert_eq!(message.translate_data().message_params().len(), 1);
        assert_eq!(param(&message, "path"), Some(path.as_str()));
        assert_eq!(param(&message, "line"), None);
        assert_eq!(param(&message, "column"), None);
        assert_eq!(
            message.extra_detail().map(ShareableString::as_str),
            Some(diagnostic.as_str())
        );
        assert_eq!(diagnostic, "unrepresentable value");
        assert_translates_everywhere(
            &message,
            "'model.hxm': Data could not be serialized as JSON",
        );
    }

    #[test]
    fn serialization_preserves_io_errors() {
        let path = Utf8Path::new("model.hxm");
        let error = io::Error::new(ErrorKind::PermissionDenied, "write denied");
        let expected = file_io_error(path, &error);

        assert_eq!(
            json_serialization_error(path, serde_json::Error::io(error)),
            expected
        );
    }

    #[test]
    fn file_io_errors_use_localized_messages_and_separate_os_details() {
        let map = generate_translation_map(&SharedStringStore::new());
        let path = Utf8Path::new("components/example.hxc");
        let cases = [
            (ErrorKind::NotFound, "files_io_not_found"),
            (ErrorKind::PermissionDenied, "files_io_permission_denied"),
            (ErrorKind::ConnectionRefused, "files_io_connection_refused"),
            (ErrorKind::ConnectionReset, "files_io_connection_reset"),
            (ErrorKind::HostUnreachable, "files_io_host_unreachable"),
            (
                ErrorKind::NetworkUnreachable,
                "files_io_network_unreachable",
            ),
            (ErrorKind::ConnectionAborted, "files_io_connection_aborted"),
            (ErrorKind::NotConnected, "files_io_not_connected"),
            (ErrorKind::AddrInUse, "files_io_address_in_use"),
            (
                ErrorKind::AddrNotAvailable,
                "files_io_address_not_available",
            ),
            (ErrorKind::NetworkDown, "files_io_network_down"),
            (ErrorKind::BrokenPipe, "files_io_broken_pipe"),
            (ErrorKind::AlreadyExists, "files_io_already_exists"),
            (ErrorKind::WouldBlock, "files_io_would_block"),
            (ErrorKind::NotADirectory, "files_io_not_a_directory"),
            (ErrorKind::IsADirectory, "files_io_not_a_file"),
            (ErrorKind::DirectoryNotEmpty, "files_io_directory_not_empty"),
            (
                ErrorKind::ReadOnlyFilesystem,
                "files_io_read_only_filesystem",
            ),
            (
                ErrorKind::StaleNetworkFileHandle,
                "files_io_stale_network_file_handle",
            ),
            (ErrorKind::InvalidInput, "files_io_invalid_input"),
            (ErrorKind::InvalidData, "files_io_invalid_data"),
            (ErrorKind::TimedOut, "files_io_timed_out"),
            (ErrorKind::WriteZero, "files_io_write_zero"),
            (ErrorKind::StorageFull, "files_io_storage_full"),
            (ErrorKind::NotSeekable, "files_io_not_seekable"),
            (ErrorKind::QuotaExceeded, "files_io_quota_exceeded"),
            (ErrorKind::FileTooLarge, "files_io_file_too_large"),
            (ErrorKind::ResourceBusy, "files_io_resource_busy"),
            (
                ErrorKind::ExecutableFileBusy,
                "files_io_executable_file_busy",
            ),
            (ErrorKind::Deadlock, "files_io_deadlock"),
            (ErrorKind::CrossesDevices, "files_io_crosses_devices"),
            (ErrorKind::TooManyLinks, "files_io_too_many_links"),
            (ErrorKind::InvalidFilename, "files_io_invalid_filename"),
            (
                ErrorKind::ArgumentListTooLong,
                "files_io_argument_list_too_long",
            ),
            (ErrorKind::Interrupted, "files_io_interrupted"),
            (ErrorKind::Unsupported, "files_io_unsupported"),
            (ErrorKind::UnexpectedEof, "files_io_unexpected_eof"),
            (ErrorKind::OutOfMemory, "files_io_out_of_memory"),
            (ErrorKind::Other, "files_io_other"),
        ];

        for (kind, key) in cases {
            let error = io::Error::new(kind, "untranslated OS diagnostic");
            let message = file_io_error(path, &error);

            assert_eq!(message.level(), MessageLevel::Error);
            assert_eq!(message.category(), MessageCategory::File);
            assert_eq!(message.translate_data().message_key().as_str(), key);
            assert_eq!(message.translate_data().message_params().len(), 1);
            assert_eq!(param(&message, "path"), Some(path.as_str()));
            assert_eq!(
                message.extra_detail().map(ShareableString::as_str),
                Some("untranslated OS diagnostic")
            );
            let english = message.translated_message(&map, "en").unwrap();
            for language in ["en", "zh", "de", "es", "fr", "ja", "ko"] {
                let translated = message.translated_message(&map, language).unwrap();
                assert!(translated.as_str().contains(path.as_str()));
                assert!(!translated.as_str().contains("untranslated OS diagnostic"));
                assert!(!translated.as_str().contains("%{"));
                if language != "en" {
                    assert_ne!(translated, english, "{key}: {language}");
                }
            }
        }
    }

    #[test]
    fn unrecognized_os_error_uses_localized_fallback() {
        let error = io::Error::from_raw_os_error(i32::MAX);
        let message = file_io_error(Utf8Path::new("example.hxc"), &error);

        assert_eq!(message.level(), MessageLevel::Error);
        assert_eq!(message.category(), MessageCategory::File);
        assert_eq!(
            message.translate_data().message_key().as_str(),
            "files_io_error"
        );
        assert_eq!(message.translate_data().message_params().len(), 1);
        assert_eq!(param(&message, "path"), Some("example.hxc"));
        assert_eq!(
            message.extra_detail().map(ShareableString::as_str),
            Some(error.to_string().as_str())
        );
        assert_translates_everywhere(&message, "File I/O error for 'example.hxc'");
    }
}
