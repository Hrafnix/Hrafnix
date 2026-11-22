use crate::traits::TreePrint;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};

/// Definition for a folder-based parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct FolderDefinition {
    /// Human-readable description of this folder parameter.
    description: ShareableString,
    /// Whether this folder should be included when archiving the project and whether file dialogs should be used to select the folder.
    is_input: bool,
    /// Default value for this folder parameter.
    default_value: ShareableString,
    /// Pre-computed BLAKE3 hash of this definition.
    hash: [u8; 32],
}

impl FolderDefinition {
    /// Creates a new folder-based `FolderDefinition`.
    pub fn new<S: Into<ShareableString>>(description: S, is_input: bool) -> Self {
        let mut s = Self {
            description: description.into(),
            is_input,
            default_value: ShareableString::default(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new folder-based `FolderDefinition` with a default value.
    pub fn new_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        is_input: bool,
        default_value: S2,
    ) -> Self {
        let mut s = Self {
            description: description.into(),
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
        h.update(b"FolderDefinition");
        h.update(&self.description.current_blake3_hash());
        h.update(&[u8::from(self.is_input)]);
        h.update(&self.default_value.current_blake3_hash());

        self.hash = *h.finalize().as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the definition.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns whether the folder should be included when archiving the project and whether file dialogs should be used to select the folder.
    #[must_use]
    pub const fn is_input(&self) -> bool {
        self.is_input
    }

    /// Returns a new `FolderDefinition` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            is_input: self.is_input,
            default_value: store.launder(&self.default_value),
            hash: self.hash,
        }
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
    /// Descriptions and defaults are ignored; the input flag must match.
    #[must_use]
    pub const fn is_merge_compatible(&self, other: &Self) -> bool {
        self.is_input == other.is_input
    }
}

impl PartialEq<&FolderDefinition> for FolderDefinition {
    fn eq(&self, other: &&FolderDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<FolderDefinition> for &FolderDefinition {
    fn eq(&self, other: &FolderDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for FolderDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Folder - default: \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value
        )
    }
}
