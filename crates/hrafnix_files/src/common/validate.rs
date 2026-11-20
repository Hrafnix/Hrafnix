use hrafnix_message::message::{Message, MessageCategory, MessageLevel};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Display;

/// Rejects the first key that repeats an earlier key, in iteration order.
///
/// `message_key` is the localized diagnostic, which receives the repeated key as `key`.
pub(crate) fn ensure_unique_keys<'a, K: Ord + Display + 'a>(
    keys: impl IntoIterator<Item = &'a K>,
    message_key: &'static str,
) -> Result<(), Message> {
    let mut seen = BTreeSet::new();
    for key in keys {
        if !seen.insert(key) {
            return Err(duplicate_key_error(message_key, key));
        }
    }
    Ok(())
}

/// Collects entries read in file order, rejecting the first repeated key.
pub(crate) fn unique_entries<K: Ord + Display, T>(
    entries: impl IntoIterator<Item = (K, T)>,
    message_key: &'static str,
) -> Result<BTreeMap<K, T>, Message> {
    let mut map = BTreeMap::new();
    for (key, value) in entries {
        if map.contains_key(&key) {
            return Err(duplicate_key_error(message_key, &key));
        }
        map.insert(key, value);
    }
    Ok(map)
}

/// Reports a repeated key with the given localized diagnostic.
fn duplicate_key_error(message_key: &'static str, key: &impl Display) -> Message {
    Message::error_with_param(MessageCategory::File, message_key, "key", key.to_string())
}

/// Rejects the first default row whose width differs from the table's column count.
pub(crate) fn validate_default_rows(
    rows: &[Vec<String>],
    column_count: usize,
) -> Result<(), Message> {
    rows.iter()
        .enumerate()
        .find(|(_, row)| row.len() != column_count)
        .map_or(Ok(()), |(index, row)| {
            Err(Message::new_with_params(
                MessageLevel::Error,
                MessageCategory::File,
                "files_table_default_row_width".into(),
                [
                    ("row".into(), index.to_string().into()),
                    ("values".into(), row.len().to_string().into()),
                    ("columns".into(), column_count.to_string().into()),
                ]
                .into(),
                None,
            ))
        })
}

/// Rejects a table whose unit list does not have one entry per column.
pub(crate) fn validate_unit_count(unit_count: usize, column_count: usize) -> Result<(), Message> {
    if unit_count == column_count {
        return Ok(());
    }
    Err(Message::new_with_params(
        MessageLevel::Error,
        MessageCategory::File,
        "files_table_unit_count_mismatch".into(),
        [
            ("units".into(), unit_count.to_string().into()),
            ("columns".into(), column_count.to_string().into()),
        ]
        .into(),
        None,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_shareable_string::{ShareableString, SharedStringStore};
    use hrafnix_translation::generate_translation_map;

    /// Checks that every supported language substitutes all parameters.
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
    fn duplicate_keys_have_localized_diagnostics() {
        let cases = [
            (
                "files_duplicate_component_id",
                "Duplicate component ID 'pump'",
            ),
            (
                "files_duplicate_parameter_key",
                "Duplicate parameter key 'pump'",
            ),
            ("files_duplicate_global_key", "Duplicate global key 'pump'"),
            (
                "files_duplicate_variable_key",
                "Duplicate variable key 'pump'",
            ),
            ("files_duplicate_column_key", "Duplicate column key 'pump'"),
            (
                "files_duplicate_map_item_key",
                "Duplicate map item key 'pump'",
            ),
            (
                "files_duplicate_map_entry_key",
                "Duplicate map entry key 'pump'",
            ),
            (
                "files_duplicate_translation_key",
                "Duplicate translation key 'pump'",
            ),
            (
                "files_duplicate_translation_language",
                "Duplicate translation language 'pump'",
            ),
        ];
        for (message_key, english) in cases {
            let keys = ["a", "pump", "b", "pump", "a"];
            let error = ensure_unique_keys(&keys, message_key).unwrap_err();
            assert_eq!(error.level(), MessageLevel::Error);
            assert_eq!(error.category(), MessageCategory::File);
            assert_eq!(error.translate_data().message_key().as_str(), message_key);
            assert_translates_everywhere(&error, english);

            let entries = keys.into_iter().map(|key| (key, ()));
            assert_eq!(unique_entries(entries, message_key).unwrap_err(), error);
        }
        assert!(ensure_unique_keys(&["a", "b"], "files_duplicate_column_key").is_ok());
        assert_eq!(
            unique_entries([("b", 2), ("a", 1)], "files_duplicate_column_key").unwrap(),
            BTreeMap::from([("a", 1), ("b", 2)])
        );
    }

    #[test]
    fn table_shape_errors_have_localized_diagnostics() {
        let rows = vec![vec!["1".to_owned(); 2], vec!["1".to_owned(); 3]];
        assert!(validate_default_rows(&rows[..1], 2).is_ok());
        let error = validate_default_rows(&rows, 2).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_table_default_row_width"
        );
        assert_translates_everywhere(
            &error,
            "Table default row 1 has 3 values but the table has 2 columns",
        );

        assert!(validate_unit_count(2, 2).is_ok());
        let error = validate_unit_count(1, 3).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_table_unit_count_mismatch"
        );
        assert_translates_everywhere(
            &error,
            "Table unit count 1 does not match its column count 3",
        );
    }
}
