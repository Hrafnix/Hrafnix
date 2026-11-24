use crate::traits::TreePrint;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};

/// Definition for a string-based parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct StringDefinition {
    /// Human-readable description of this string parameter.
    description: ShareableString,
    /// Default value for this string parameter.
    default_value: ShareableString,
    /// Pre-computed BLAKE3 hash of this definition.
    hash: [u8; 32],
}

impl StringDefinition {
    /// Creates a new string-based `StringDefinition`.
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        let mut s = Self {
            description: description.into(),
            default_value: ShareableString::default(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new string-based `StringDefinition` with a default value.
    pub fn new_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        default_value: S2,
    ) -> Self {
        let mut s = Self {
            description: description.into(),
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
        h.update(b"StringDefinition");
        h.update(&self.description.current_blake3_hash());
        h.update(&self.default_value.current_blake3_hash());

        self.hash = *h.finalize().as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the definition.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns a new `StringDefinition` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
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
}

impl PartialEq<&StringDefinition> for StringDefinition {
    fn eq(&self, other: &&StringDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<StringDefinition> for &StringDefinition {
    fn eq(&self, other: &StringDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for StringDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) String - default: \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value,
        )
    }
}
