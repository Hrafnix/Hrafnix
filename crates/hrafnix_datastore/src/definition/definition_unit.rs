use crate::traits::TreePrint;
use hrafnix_keys::UnitKey;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};
use hrafnix_units::{UnitFamilyId, UnitId};

/// Definition for a choice-based parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct UnitDefinition {
    /// Human-readable description of this choice parameter.
    description: ShareableString,
    /// Unit family
    unit_family: UnitFamilyId,
    /// Default value for this choice parameter.
    default_value: ShareableString,
    /// Pre-computed BLAKE3 hash of this definition.
    hash: [u8; 32],
}

impl UnitDefinition {
    /// Creates a new `UnitDefinition` with the specified choices.
    pub fn new<S: Into<ShareableString>>(description: S, unit_family: UnitFamilyId) -> Self {
        let mut s = Self {
            description: description.into(),
            unit_family,
            default_value: ShareableString::default(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `UnitDefinition` with the specified choices and default value.
    pub fn new_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        unit_family: UnitFamilyId,
        default_value: S2,
    ) -> Self {
        let mut s = Self {
            description: description.into(),
            unit_family,
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
        h.update(b"UnitDefinition");
        h.update(&self.description.current_blake3_hash());
        h.update(&[self.unit_family.to_u8()]);
        h.update(&self.default_value.current_blake3_hash());

        self.hash = *h.finalize().as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the definition.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns a reference to the unit family.
    #[must_use]
    pub const fn unit_family(&self) -> UnitFamilyId {
        self.unit_family
    }

    /// Returns true if the given value is a valid units.
    pub fn contains<S: Into<ShareableString>>(&self, value: S) -> bool {
        let value = value.into();
        self.unit_family
            .unit_ids()
            .iter()
            .any(|unit_id| unit_id.string_id().as_str() == value)
    }

    /// Returns a vector of keys `UnitKey` for the units.
    #[must_use]
    pub fn keys(&self) -> Vec<UnitKey> {
        self.unit_family
            .unit_ids()
            .iter()
            .map(|unit_id| unit_id.string_id().into())
            .collect()
    }

    /// Returns a vector of Unit IDs for the units.
    #[must_use]
    pub fn ids(&self) -> Vec<UnitId> {
        self.unit_family.unit_ids().to_vec()
    }

    /// Returns a vector of descriptions for the units.
    #[must_use]
    pub fn descriptions(&self) -> Vec<ShareableString> {
        self.unit_family
            .unit_ids()
            .iter()
            .map(|unit_id| unit_id.description().into())
            .collect()
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
    /// Descriptions and defaults are ignored; the unit family must match.
    #[must_use]
    pub const fn is_merge_compatible(&self, other: &Self) -> bool {
        self.unit_family.to_u8() == other.unit_family.to_u8()
    }

    /// Interns all strings in `store` in place, preserving values, hashes, and ordering.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.description = store.launder(&self.description);
        self.default_value = store.launder(&self.default_value);
    }
}

impl PartialEq<&UnitDefinition> for UnitDefinition {
    fn eq(&self, other: &&UnitDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<UnitDefinition> for &UnitDefinition {
    fn eq(&self, other: &UnitDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for UnitDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Unit - default: \"{}\" [{}]",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value,
            self.ids()
                .iter()
                .map(|unit_id| format!("{} ({})", unit_id.string_id(), unit_id.description()))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
