use crate::definition::UnitDefinition;
use crate::editable::UnitEditable;
use crate::traits::TreePrint;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};

/// Represents a unit data value in the frozen data.
#[derive(Debug, Clone, PartialEq)]
pub struct UnitFrozen {
    /// Definition metadata for this unit value.
    definition: UnitDefinition,
    /// Current unevaluated value for this unit data, stored as a `ShareableString`.
    value: ShareableString,
    /// Pre-computed BLAKE3 hash of the value for fast diffing.
    hash: [u8; 32],
}

impl UnitFrozen {
    /// Creates a new `UnitFrozen` instance.
    #[must_use]
    pub fn new(definition: UnitDefinition) -> Self {
        let value = definition.default_value();

        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `UnitFrozen` instance with a specified value.
    #[must_use]
    pub fn new_with_value<S: Into<ShareableString>>(definition: UnitDefinition, value: S) -> Self {
        let mut s = Self {
            definition,
            value: value.into(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `UnitFrozen` instance from a given `UnitEditable` value.
    #[must_use]
    pub fn new_from_editable(basic: &UnitEditable) -> Self {
        let definition = basic.definition().clone();
        let value = basic.value();
        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Converts the current `UnitFrozen` instance into a `UnitEditable` instance.
    #[must_use]
    pub fn thaw(&self) -> UnitEditable {
        UnitEditable::new(self)
    }

    /// Recomputes and stores the BLAKE3 hash of the current value.
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"Unit");

        h.update(&self.value.current_blake3_hash());

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the unevaluated value as a `ShareableString`.
    #[must_use]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the unit definition.
    #[must_use]
    pub const fn definition(&self) -> &UnitDefinition {
        &self.definition
    }

    /// Returns the pre-calculated BLAKE3 hash of the value.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Copies the value from `other` into `self` if their definitions are merge compatible.
    ///
    /// `self` keeps its own definition. Returns true if `self` was modified.
    pub(crate) fn update_value_from(&mut self, other: &Self) -> bool {
        if !self.definition.is_merge_compatible(&other.definition) || self.value == other.value {
            return false;
        }
        self.value.clone_from(&other.value);
        self.update_hash();
        true
    }

    /// Interns all strings in `store` in place, preserving values, hashes, and ordering.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.definition.launder(store);
        self.value = store.launder(&self.value);
    }
}

impl PartialEq<&UnitFrozen> for UnitFrozen {
    fn eq(&self, other: &&UnitFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<UnitFrozen> for &UnitFrozen {
    fn eq(&self, other: &UnitFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for UnitFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Unit - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
