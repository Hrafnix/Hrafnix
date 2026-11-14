use hrafnix_shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Source language and translated text shared by model and component files.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranslationsFile {
    /// Source language of the translation keys.
    pub language: ShareableString,
    /// Original text mapped to target language codes and their translated text.
    pub translations: BTreeMap<ShareableString, BTreeMap<ShareableString, ShareableString>>,
}
