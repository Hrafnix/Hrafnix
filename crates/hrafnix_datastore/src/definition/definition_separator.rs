use crate::traits::TreePrint;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};

/// Definition for a separator-based parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct SeparatorDefinition {
    /// Human-readable description of this separator parameter.
    description: ShareableString,
    /// Pre-computed BLAKE3 hash of this definition.
    hash: [u8; 32],
}

impl SeparatorDefinition {
    /// Creates a new separator-based `SeparatorDefinition`.
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        let mut s = Self {
            description: description.into(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Computes and stores the BLAKE3 hash of this definition.
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        h.update(&[0x01]);
        h.update(b"SeparatorDefinition");
        h.update(&self.description.current_blake3_hash());

        self.hash = *h.finalize().as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the definition.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns a new `SeparatorDefinition` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
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
}

impl PartialEq<&SeparatorDefinition> for SeparatorDefinition {
    fn eq(&self, other: &&SeparatorDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<SeparatorDefinition> for &SeparatorDefinition {
    fn eq(&self, other: &SeparatorDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for SeparatorDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Separator",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
        )
    }
}
