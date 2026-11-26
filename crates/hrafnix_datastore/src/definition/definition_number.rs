use crate::definition::{NumberConstraint, NumberConstraintEnum};
use crate::traits::TreePrint;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};

/// Definition for a number-based parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct NumberDefinition {
    /// Human-readable description of this number parameter.
    description: ShareableString,
    /// Optional constraint (min, max, range, or none) applied to the value.
    constraint: NumberConstraint,
    /// Default value for this number parameter.
    default_value: ShareableString,
    /// Pre-computed BLAKE3 hash of this definition.
    hash: [u8; 32],
}

impl NumberDefinition {
    /// Creates a new number-based `NumberDefinition`.
    pub fn new<S1: Into<ShareableString>>(description: S1) -> Self {
        let mut s = Self {
            description: description.into(),
            constraint: NumberConstraint::none(),
            default_value: ShareableString::default(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new number-based `NumberDefinition` with a default value.
    pub fn new_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        default_value: S2,
    ) -> Self {
        let mut s = Self {
            description: description.into(),
            constraint: NumberConstraint::none(),
            default_value: default_value.into(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new number-based `NumberDefinition`.
    pub fn new_with_constraint<S: Into<ShareableString>>(
        description: S,
        constraint: NumberConstraint,
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

    /// Creates a new number-based `NumberDefinition` with a default value.
    pub fn new_with_constraint_and_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        constraint: NumberConstraint,
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
        h.update(b"NumberDefinition");
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
    pub fn constraint(&self) -> NumberConstraintEnum {
        self.constraint.constraint_enum.clone()
    }

    /// Returns a reference to the constraint.
    #[must_use]
    pub const fn constraint_ref(&self) -> &NumberConstraintEnum {
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

impl PartialEq<&NumberDefinition> for NumberDefinition {
    fn eq(&self, other: &&NumberDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<NumberDefinition> for &NumberDefinition {
    fn eq(&self, other: &NumberDefinition) -> bool {
        *self == other
    }
}

/// Formats an `f64` for display, appending `.0` when the value has no
/// fractional part and is not in scientific notation.
fn format_number_value(value: f64) -> String {
    if !value.is_finite() {
        return value.to_string();
    }

    let mut formatted = value.to_string();
    if !formatted.contains('.') && !formatted.contains('e') && !formatted.contains('E') {
        formatted.push_str(".0");
    }

    formatted
}

impl TreePrint for NumberDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        let constraint_str = match &self.constraint.constraint_enum {
            NumberConstraintEnum::Min { min, inclusive } => {
                format!(
                    " [Min({}, {})]",
                    format_number_value(*min),
                    if *inclusive { "inclusive" } else { "exclusive" }
                )
            }
            NumberConstraintEnum::Max { max, inclusive } => {
                format!(
                    " [Max({}, {})]",
                    format_number_value(*max),
                    if *inclusive { "inclusive" } else { "exclusive" }
                )
            }
            NumberConstraintEnum::Range {
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
                format!(
                    " [Range({}, {}, {}, {})]",
                    format_number_value(*min),
                    format_number_value(*max),
                    min_type,
                    max_type
                )
            }
            NumberConstraintEnum::None => String::new(),
        };

        writeln!(
            f,
            "{}{}{} ({}) Number - default: \"{}\"{}",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value,
            constraint_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::format_number_value;

    #[test]
    fn format_number_value_keeps_fractional_values() {
        assert_eq!(format_number_value(1.52), "1.52");
    }

    #[test]
    fn format_number_value_adds_single_decimal_for_integers() {
        assert_eq!(format_number_value(1.0), "1.0");
        assert_eq!(format_number_value(42.0), "42.0");
    }
}
