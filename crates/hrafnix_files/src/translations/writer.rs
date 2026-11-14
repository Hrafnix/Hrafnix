use crate::common::serialize::serialize_shareable_string;
use crate::translations::data::TranslationsFile;
use hrafnix_shareable_string::ShareableString;
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

/// The current JSON representation of the `translations` entry used when writing.
#[derive(Debug, Serialize)]
pub(crate) struct TranslationsWriter {
    /// Source language of the translation keys.
    #[serde(serialize_with = "serialize_shareable_string")]
    language: ShareableString,
    /// Original text mapped to target languages and their translations.
    #[serde(serialize_with = "serialize_translations")]
    translations: BTreeMap<ShareableString, BTreeMap<ShareableString, ShareableString>>,
}

/// Serializes both map levels in key order without copying their string contents.
fn serialize_translations<S: Serializer>(
    translations: &BTreeMap<ShareableString, BTreeMap<ShareableString, ShareableString>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    translations
        .iter()
        .map(|(key, languages)| {
            (
                key.as_str(),
                languages
                    .iter()
                    .map(|(language, value)| (language.as_str(), value.as_str()))
                    .collect::<BTreeMap<_, _>>(),
            )
        })
        .collect::<BTreeMap<_, _>>()
        .serialize(serializer)
}

impl From<&TranslationsFile> for TranslationsWriter {
    fn from(translations: &TranslationsFile) -> Self {
        Self {
            language: translations.language.clone(),
            translations: translations.translations.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TranslationsWriter;
    use crate::common::deserialize::RootObject;
    use crate::translations::data::TranslationsFile;
    use crate::translations::reader::v1::TranslationsReaderV1;
    use serde_json::{Value, json};
    use std::collections::BTreeMap;

    #[test]
    fn translations_match_the_nested_reader_schema() {
        for language in ["", "en", "fr-CA"] {
            for translations in [
                BTreeMap::new(),
                [
                    (
                        "Target temperature".into(),
                        [
                            ("es".into(), "Temperatura objetivo".into()),
                            ("fr".into(), "Temp\u{e9}rature cible".into()),
                            ("de".into(), "Zieltemperatur".into()),
                        ]
                        .into(),
                    ),
                    ("empty".into(), BTreeMap::new()),
                    ("quoted \"key\"\n".into(), [("".into(), "".into())].into()),
                ]
                .into(),
            ] {
                let data = TranslationsFile {
                    language: language.into(),
                    translations,
                };
                let json = serde_json::to_string(&TranslationsWriter::from(&data)).unwrap();
                let expected = if data.translations.is_empty() {
                    json!({})
                } else {
                    json!({
                        "Target temperature": {
                            "es": "Temperatura objetivo",
                            "fr": "Temp\u{e9}rature cible",
                            "de": "Zieltemperatur",
                        },
                        "empty": {},
                        "quoted \"key\"\n": {"": ""},
                    })
                };
                assert_eq!(
                    serde_json::from_str::<Value>(&json).unwrap(),
                    json!({"language": language, "translations": expected}),
                );
                let reader: RootObject<TranslationsReaderV1> = serde_json::from_str(&json).unwrap();
                assert_eq!(TranslationsFile::from(reader.document), data);
            }
        }
    }

    #[test]
    fn translation_keys_and_language_codes_are_written_in_sorted_order() {
        let data = TranslationsFile {
            language: "en".into(),
            translations: [
                ("z".into(), [("fr".into(), "texte".into())].into()),
                (
                    "a".into(),
                    [("fr".into(), "texte".into()), ("de".into(), "Text".into())].into(),
                ),
            ]
            .into(),
        };
        assert_eq!(
            serde_json::to_string(&TranslationsWriter::from(&data)).unwrap(),
            r#"{"language":"en","translations":{"a":{"de":"Text","fr":"texte"},"z":{"fr":"texte"}}}"#,
        );
    }
}
