use crate::common::deserialize::deserialize_entries;
use crate::translations::data::TranslationsFile;
use serde::Deserialize;
use std::collections::BTreeMap;

/// The version 1 JSON representation of the `translations` entry.
///
/// Every field is required. Unknown fields and duplicate translation keys or
/// language codes are rejected so nothing is silently dropped when writing back.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TranslationsReaderV1 {
    /// Source language of the translation keys.
    language: String,
    /// Original text mapped to target languages and their translations.
    #[serde(deserialize_with = "deserialize_entries")]
    translations: BTreeMap<String, LanguageTranslationsReaderV1>,
}

/// Target language codes mapped to translated text for one translation key.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
struct LanguageTranslationsReaderV1 {
    /// Translations with unique language codes and string values.
    #[serde(deserialize_with = "deserialize_entries")]
    translations: BTreeMap<String, String>,
}

impl From<TranslationsReaderV1> for TranslationsFile {
    fn from(translations: TranslationsReaderV1) -> Self {
        Self {
            language: translations.language.into(),
            translations: translations
                .translations
                .into_iter()
                .map(|(key, languages)| {
                    (
                        key.into(),
                        languages
                            .translations
                            .into_iter()
                            .map(|(language, value)| (language.into(), value.into()))
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TranslationsReaderV1;
    use crate::common::deserialize::RootObject;
    use crate::translations::data::TranslationsFile;
    use serde_json::{Value, json};

    /// Reads translations through their object-only envelope.
    fn read(value: &Value) -> Result<TranslationsFile, serde_json::Error> {
        let reader: RootObject<TranslationsReaderV1> =
            serde_json::from_str(&serde_json::to_string(value)?)?;
        Ok(reader.document.into())
    }

    /// Creates translations with an explicit source language and translation table.
    fn translations_json() -> Value {
        json!({
            "language": "en",
            "translations": {
                "Target temperature": {
                    "es": "Temperatura objetivo",
                    "fr": "Temp\u{e9}rature cible",
                    "de": "Zieltemperatur",
                },
            },
        })
    }

    #[test]
    fn source_language_keys_and_values_are_preserved() {
        let translations = read(&translations_json()).unwrap();
        assert_eq!(translations.language.as_str(), "en");
        let languages = translations.translations.get("Target temperature").unwrap();
        assert_eq!(
            languages.get("es").unwrap().as_str(),
            "Temperatura objetivo"
        );
        assert_eq!(
            languages.get("fr").unwrap().as_str(),
            "Temp\u{e9}rature cible"
        );
        assert_eq!(languages.get("de").unwrap().as_str(), "Zieltemperatur");
    }

    #[test]
    fn empty_maps_and_arbitrary_strings_are_preserved() {
        for language in ["", "en-US", " leading and trailing spaces "] {
            let translations = read(&json!({
                "language": language,
                "translations": {},
            }))
            .unwrap();
            assert_eq!(translations.language.as_str(), language);
            assert!(translations.translations.is_empty());
        }
        let translations = read(&json!({
            "language": "en",
            "translations": {
                "": {"": ""},
                " untranslated ": {},
                "quoted \"key\"\n": {"fr-CA": " texte \n\u{e9} "},
            },
        }))
        .unwrap();
        assert_eq!(translations.translations[""][""].as_str(), "");
        assert!(translations.translations[" untranslated "].is_empty());
        assert_eq!(
            translations.translations["quoted \"key\"\n"]["fr-CA"].as_str(),
            " texte \n\u{e9} ",
        );
    }

    #[test]
    fn fields_are_required_and_unknown_fields_are_rejected() {
        for field in ["language", "translations"] {
            let mut value = translations_json();
            value.as_object_mut().unwrap().remove(field);
            assert!(read(&value).is_err(), "{value}");
        }
        let mut value = translations_json();
        value["extra"] = json!(1);
        assert!(read(&value).is_err());
    }

    #[test]
    fn language_and_translation_values_require_strings() {
        for invalid in [Value::Null, json!(123), json!(false), json!([]), json!({})] {
            for is_language in [true, false] {
                let mut value = translations_json();
                if is_language {
                    value["language"] = invalid.clone();
                } else {
                    value["translations"]["Target temperature"]["es"] = invalid.clone();
                }
                let error = read(&value).unwrap_err();
                assert!(error.is_data(), "{value}: {error}");
                assert!(error.line() > 0);
                assert!(error.column() > 0);
            }
        }
    }

    #[test]
    fn structs_and_translation_maps_require_objects() {
        for invalid in [
            Value::Null,
            json!(false),
            json!(123),
            json!("translations"),
            json!([]),
            json!(["en", {}]),
            json!([["es", "Temperatura objetivo"]]),
        ] {
            assert!(read(&invalid).is_err(), "{invalid}");
            for is_table in [true, false] {
                let mut value = translations_json();
                if is_table {
                    value["translations"] = invalid.clone();
                } else {
                    value["translations"]["Target temperature"] = invalid.clone();
                }
                assert!(read(&value).is_err(), "{value}");
            }
        }
    }

    #[test]
    fn duplicate_fields_keys_and_languages_are_rejected() {
        for json in [
            r#"{"language":"en","language":"es","translations":{}}"#,
            r#"{"language":"en","translations":{},"translations":{}}"#,
            r#"{"language":"en","translations":{"Target temperature":{},"Target temperature":{}}}"#,
            r#"{"language":"en","translations":{"key":{},"\u006bey":{}}}"#,
            r#"{"language":"en","translations":{"Target temperature":{"es":"a","es":"b"}}}"#,
            r#"{"language":"en","translations":{"key":{"es":"a","\u0065s":"b"}}}"#,
        ] {
            let error = serde_json::from_str::<RootObject<TranslationsReaderV1>>(json).unwrap_err();
            assert!(error.is_data(), "{json}: {error}");
            assert!(error.to_string().contains("duplicate"), "{json}: {error}");
        }
    }

    #[test]
    fn source_language_can_follow_the_translation_table() {
        let reader: RootObject<TranslationsReaderV1> =
            serde_json::from_str(r#"{"translations":{"key":{"fr":"texte"}},"language":"en"}"#)
                .unwrap();
        let translations = TranslationsFile::from(reader.document);
        assert_eq!(translations.language.as_str(), "en");
        assert_eq!(translations.translations["key"]["fr"].as_str(), "texte");
    }
}
