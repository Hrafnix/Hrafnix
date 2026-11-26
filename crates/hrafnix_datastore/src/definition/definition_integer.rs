use crate::traits::TreePrint;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};

/// Definition for a number-based parameter constraint.
#[derive(Debug, Clone, PartialEq)]
pub enum IntegerConstraintEnum {
    /// Minimum value constraint.
    Min {
        /// Minimum value of the constraint.
        min: i64,
        /// Whether the minimum value is inclusive.
        inclusive: bool,
    },
    /// Maximum value constraint.
    Max {
        /// Maximum value of the constraint.
        max: i64,
        /// Whether the maximum value is inclusive.
        inclusive: bool,
    },
    /// Range value constraint.
    Range {
        /// Minimum value of the range.
        min: i64,
        /// Maximum value of the range.
        max: i64,
        /// Whether the minimum value is inclusive.
        min_inclusive: bool,
        /// Whether the maximum value is inclusive.
        max_inclusive: bool,
    },
    /// No constraint.
    None,
}

/// Definition for an integer-based parameter constraint.
#[derive(Debug, Clone, PartialEq)]
pub struct IntegerConstraint {
    /// The actual constraint variant (none, min, max, or range).
    pub(crate) constraint_enum: IntegerConstraintEnum,
}

impl IntegerConstraint {
    /// Creates a new `IntegerConstraint` with no constraint.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            constraint_enum: IntegerConstraintEnum::None,
        }
    }

    /// Creates a new `IntegerConstraint` with a minimum value constraint.
    #[must_use]
    pub const fn min(min: i64, inclusive: bool) -> Self {
        Self {
            constraint_enum: IntegerConstraintEnum::Min { min, inclusive },
        }
    }

    /// Creates a new `IntegerConstraint` with a maximum value constraint.
    #[must_use]
    pub const fn max(max: i64, inclusive: bool) -> Self {
        Self {
            constraint_enum: IntegerConstraintEnum::Max { max, inclusive },
        }
    }

    /// Creates a new `IntegerConstraint` with a range value constraint.
    ///
    /// If `value_1` is greater than `value_2`, the two values are swapped along with
    /// their corresponding inclusivity flags, so the resulting range is always valid.
    ///
    /// If `value_1` and `value_2` are equal, the resulting range is always inclusive on
    /// both ends (regardless of the passed inclusivity flags), so it always represents
    /// exactly that single value rather than a contradictory, unsatisfiable range.
    #[must_use]
    pub fn range(
        value_1: i64,
        value_2: i64,
        value_1_inclusive: bool,
        value_2_inclusive: bool,
    ) -> Self {
        let (min, max, min_inclusive, max_inclusive) = match value_1.cmp(&value_2) {
            std::cmp::Ordering::Equal => (value_1, value_2, true, true),
            std::cmp::Ordering::Greater => (value_2, value_1, value_2_inclusive, value_1_inclusive),
            std::cmp::Ordering::Less => (value_1, value_2, value_1_inclusive, value_2_inclusive),
        };

        Self {
            constraint_enum: IntegerConstraintEnum::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            },
        }
    }

    /// Feeds this constraint into a BLAKE3 hasher.
    pub(crate) fn update_hasher(&self, h: &mut blake3::Hasher) {
        match self.constraint_enum {
            IntegerConstraintEnum::None => {
                h.update(&[0x00]);
            }
            IntegerConstraintEnum::Min { min, inclusive } => {
                h.update(&[0x01]);
                h.update(&min.to_le_bytes());
                h.update(&[u8::from(inclusive)]);
            }
            IntegerConstraintEnum::Max { max, inclusive } => {
                h.update(&[0x02]);
                h.update(&max.to_le_bytes());
                h.update(&[u8::from(inclusive)]);
            }
            IntegerConstraintEnum::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => {
                h.update(&[0x03]);
                h.update(&min.to_le_bytes());
                h.update(&max.to_le_bytes());
                h.update(&[u8::from(min_inclusive), u8::from(max_inclusive)]);
            }
        }
    }
}

/// Definition for an integer-based parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct IntegerDefinition {
    /// Human-readable description of this integer parameter.
    description: ShareableString,
    /// Optional constraint (min, max, range, or none) applied to the value.
    constraint: IntegerConstraint,
    /// Default value for this integer parameter.
    default_value: ShareableString,
    /// Pre-computed BLAKE3 hash of this definition.
    hash: [u8; 32],
}

impl IntegerDefinition {
    /// Creates a new integer-based `IntegerDefinition`.
    pub fn new<S1: Into<ShareableString>>(description: S1) -> Self {
        let mut s = Self {
            description: description.into(),
            constraint: IntegerConstraint::none(),
            default_value: ShareableString::default(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new integer-based `IntegerDefinition` with a default value.
    pub fn new_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        default_value: S2,
    ) -> Self {
        let mut s = Self {
            description: description.into(),
            constraint: IntegerConstraint::none(),
            default_value: default_value.into(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new integer-based `IntegerDefinition`.
    pub fn new_with_constraint<S: Into<ShareableString>>(
        description: S,
        constraint: IntegerConstraint,
    ) -> Self {
        let mut s = Self {
            description: description.into(),
            constraint,
            default_value: ShareableString::default(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new integer-based `IntegerDefinition` with a default value.
    pub fn new_with_constraint_and_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        constraint: IntegerConstraint,
        default_value: S2,
    ) -> Self {
        let mut s = Self {
            description: description.into(),
            constraint,
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
        h.update(b"IntegerDefinition");
        h.update(&self.description.current_blake3_hash());
        self.constraint.update_hasher(&mut h);
        h.update(&self.default_value.current_blake3_hash());

        self.hash = *h.finalize().as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the definition.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns the constraint.
    #[must_use]
    pub fn constraint(&self) -> IntegerConstraintEnum {
        self.constraint.constraint_enum.clone()
    }

    /// Returns a reference to the constraint.
    #[must_use]
    pub const fn constraint_ref(&self) -> &IntegerConstraintEnum {
        &self.constraint.constraint_enum
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

    /// Interns all strings in `store` in place, preserving values, hashes, and ordering.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.description = store.launder(&self.description);
        self.default_value = store.launder(&self.default_value);
    }
}

impl PartialEq<&IntegerDefinition> for IntegerDefinition {
    fn eq(&self, other: &&IntegerDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<IntegerDefinition> for &IntegerDefinition {
    fn eq(&self, other: &IntegerDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for IntegerDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        let constraint_str = match &self.constraint.constraint_enum {
            IntegerConstraintEnum::Min { min, inclusive } => {
                format!(
                    " [Min({}, {})]",
                    *min,
                    if *inclusive { "inclusive" } else { "exclusive" }
                )
            }
            IntegerConstraintEnum::Max { max, inclusive } => {
                format!(
                    " [Max({}, {})]",
                    *max,
                    if *inclusive { "inclusive" } else { "exclusive" }
                )
            }
            IntegerConstraintEnum::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => {
                let min_type = if *min_inclusive {
                    "inclusive"
                } else {
                    "exclusive"
                };
                let max_type = if *max_inclusive {
                    "inclusive"
                } else {
                    "exclusive"
                };
                format!(" [Range({}, {}, {}, {})]", *min, *max, min_type, max_type)
            }
            IntegerConstraintEnum::None => String::new(),
        };

        writeln!(
            f,
            "{}{}{} ({}) Integer - default: \"{}\"{}",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value,
            constraint_str
        )
    }
}
