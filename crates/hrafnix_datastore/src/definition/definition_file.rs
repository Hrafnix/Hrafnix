use crate::traits::TreePrint;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};

/// Definition for a file-based parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct FileDefinition {
    /// Human-readable description of this file parameter.
    description: ShareableString,
    /// File-extension filter (e.g., `"*.csv"`) used in open-file dialogs.
    extension_filter: ShareableString,
    /// Whether this file should be included when archiving the project and whether file dialogs should be used to select the file.
    is_input: bool,
    /// Default value for this file parameter.
    default_value: ShareableString,
    /// Pre-computed BLAKE3 hash of this definition.
    hash: [u8; 32],
}

impl FileDefinition {
    /// Creates a new file-based `FileDefinition`.
    pub fn new<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        extension_filter: S2,
        is_input: bool,
    ) -> Self {
        let mut s = Self {
            description: description.into(),
            extension_filter: extension_filter.into(),
            is_input,
            default_value: ShareableString::default(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new file-based `FileDefinition` with a default value.
    pub fn new_with_default<
        S1: Into<ShareableString>,
        S2: Into<ShareableString>,
        S3: Into<ShareableString>,
    >(
        description: S1,
        extension_filter: S2,
        is_input: bool,
        default_value: S3,
    ) -> Self {
        let mut s = Self {
            description: description.into(),
            extension_filter: extension_filter.into(),
            is_input,
            default_value: default_value.into(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Computes and stores the BLAKE3 hash of this definition.
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        h.update(&[0x01]);
        h.update(b"FileDefinition");
        h.update(&self.description.current_blake3_hash());
        h.update(&self.extension_filter.current_blake3_hash());
        h.update(&[u8::from(self.is_input)]);
        h.update(&self.default_value.current_blake3_hash());

        self.hash = *h.finalize().as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the definition.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns the extension filter.
    #[must_use]
    pub fn extension_filter(&self) -> ShareableString {
        self.extension_filter.clone()
    }

    /// Returns a reference to the extension filter.
    #[must_use]
    pub const fn extension_filter_ref(&self) -> &ShareableString {
        &self.extension_filter
    }

    /// Returns whether the file should be bundled when archived and whether file dialogs should be used to select the file.
    #[must_use]
    pub const fn is_input(&self) -> bool {
        self.is_input
    }

    /// Returns the description of the parameter.
    #[must_use]
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the description.
    #[must_use]
    pub const fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns the default value of the parameter.
    #[must_use]
    pub fn default_value(&self) -> ShareableString {
        self.default_value.clone()
    }

    /// Returns a reference to the default value.
    #[must_use]
    pub const fn default_value_ref(&self) -> &ShareableString {
        &self.default_value
    }

    /// Returns true if values of `other` can safely replace values of `self` during a merge.
    ///
    /// Descriptions and defaults are ignored; the extension filter and input flag must match.
    #[must_use]
    pub fn is_merge_compatible(&self, other: &Self) -> bool {
        self.extension_filter == other.extension_filter && self.is_input == other.is_input
    }

    /// Interns all strings in `store` in place, preserving values, hashes, and ordering.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.description = store.launder(&self.description);
        self.extension_filter = store.launder(&self.extension_filter);
        self.default_value = store.launder(&self.default_value);
    }
}

impl PartialEq<&FileDefinition> for FileDefinition {
    fn eq(&self, other: &&FileDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<FileDefinition> for &FileDefinition {
    fn eq(&self, other: &FileDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for FileDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) File - default: \"{}\" [{}]",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value,
            self.extension_filter
        )
    }
}
