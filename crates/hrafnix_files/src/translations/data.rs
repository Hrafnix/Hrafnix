use hrafnix_shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Source language and translated text shared by model and component files.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranslationsFile {
    /// Source language of the translation keys.
    language: ShareableString,
    /// Original text mapped to target language codes and their translated text.
    translations: BTreeMap<ShareableString, BTreeMap<ShareableString, ShareableString>>,
}

impl TranslationsFile {
    /// Creates a new translations table for the given source language.
    #[must_use]
    pub(crate) const fn new(
        language: ShareableString,
        translations: BTreeMap<ShareableString, BTreeMap<ShareableString, ShareableString>>,
    ) -> Self {
        Self {
            language,
            translations,
        }
    }

    /// Returns the source language of the translation keys.
    #[must_use]
    pub const fn language(&self) -> &ShareableString {
        &self.language
    }

    /// Returns the mapping from original text to target language translations.
    #[must_use]
    pub const fn translations(
        &self,
    ) -> &BTreeMap<ShareableString, BTreeMap<ShareableString, ShareableString>> {
        &self.translations
    }
}
