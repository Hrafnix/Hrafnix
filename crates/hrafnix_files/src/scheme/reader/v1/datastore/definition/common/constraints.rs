use crate::common::deserialize::StringTagDeserializer;
use hrafnix_datastore::definition::{IntegerConstraint, NumberConstraint};
use serde::{Deserialize, Deserializer};

/// Version-1 integer constraint, tagged by `type`.
#[derive(Debug, Deserialize)]
#[serde(
    remote = "Self",
    tag = "type",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub(crate) enum ConstraintIntDefinitionReaderV1 {
    /// No bounds.
    None {},
    /// Lower bound.
    Min {
        /// Minimum value.
        min: i64,
        /// Whether the minimum is allowed.
        inclusive: bool,
    },
    /// Upper bound.
    Max {
        /// Maximum value.
        max: i64,
        /// Whether the maximum is allowed.
        inclusive: bool,
    },
    /// Lower and upper bounds.
    Range {
        /// Minimum value.
        min: i64,
        /// Maximum value.
        max: i64,
        /// Whether the minimum is allowed.
        min_inclusive: bool,
        /// Whether the maximum is allowed.
        max_inclusive: bool,
    },
}

impl<'de> Deserialize<'de> for ConstraintIntDefinitionReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::deserialize(StringTagDeserializer(deserializer))
    }
}

impl From<ConstraintIntDefinitionReaderV1> for IntegerConstraint {
    fn from(constraint: ConstraintIntDefinitionReaderV1) -> Self {
        match constraint {
            ConstraintIntDefinitionReaderV1::None {} => Self::none(),
            ConstraintIntDefinitionReaderV1::Min { min, inclusive } => Self::min(min, inclusive),
            ConstraintIntDefinitionReaderV1::Max { max, inclusive } => Self::max(max, inclusive),
            ConstraintIntDefinitionReaderV1::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => Self::range(min, max, min_inclusive, max_inclusive),
        }
    }
}

/// Version-1 number constraint, tagged by `type`.
#[derive(Debug, Deserialize)]
#[serde(
    remote = "Self",
    tag = "type",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub(crate) enum ConstraintNumberDefinitionReaderV1 {
    /// No bounds.
    None {},
    /// Lower bound.
    Min {
        /// Minimum value.
        min: f64,
        /// Whether the minimum is allowed.
        inclusive: bool,
    },
    /// Upper bound.
    Max {
        /// Maximum value.
        max: f64,
        /// Whether the maximum is allowed.
        inclusive: bool,
    },
    /// Lower and upper bounds.
    Range {
        /// Minimum value.
        min: f64,
        /// Maximum value.
        max: f64,
        /// Whether the minimum is allowed.
        min_inclusive: bool,
        /// Whether the maximum is allowed.
        max_inclusive: bool,
    },
}

impl<'de> Deserialize<'de> for ConstraintNumberDefinitionReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::deserialize(StringTagDeserializer(deserializer))
    }
}

impl From<ConstraintNumberDefinitionReaderV1> for NumberConstraint {
    fn from(constraint: ConstraintNumberDefinitionReaderV1) -> Self {
        match constraint {
            ConstraintNumberDefinitionReaderV1::None {} => Self::none(),
            ConstraintNumberDefinitionReaderV1::Min { min, inclusive } => Self::min(min, inclusive),
            ConstraintNumberDefinitionReaderV1::Max { max, inclusive } => Self::max(max, inclusive),
            ConstraintNumberDefinitionReaderV1::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => Self::range(min, max, min_inclusive, max_inclusive),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use serde_json::json;

    #[test]
    fn reads_every_integer_constraint() {
        for (value, expected) in [
            (json!({"type": "none"}), IntegerConstraint::none()),
            (
                json!({"type": "min", "min": -3, "inclusive": false}),
                IntegerConstraint::min(-3, false),
            ),
            (
                json!({"type": "max", "max": 8, "inclusive": true}),
                IntegerConstraint::max(8, true),
            ),
            (
                json!({
                    "type": "range", "min": -3, "max": 8,
                    "min_inclusive": false, "max_inclusive": true
                }),
                IntegerConstraint::range(-3, 8, false, true),
            ),
        ] {
            let reader =
                serde_json::from_value::<RootObject<ConstraintIntDefinitionReaderV1>>(value)
                    .unwrap();
            assert_eq!(IntegerConstraint::from(reader.document), expected);
        }
    }

    #[test]
    fn reads_every_number_constraint_and_normalizes_reversed_bounds() {
        for (value, expected) in [
            (json!({"type": "none"}), NumberConstraint::none()),
            (
                json!({"type": "min", "min": -1.5, "inclusive": false}),
                NumberConstraint::min(-1.5, false),
            ),
            (
                json!({"type": "max", "max": 2.5, "inclusive": true}),
                NumberConstraint::max(2.5, true),
            ),
            (
                json!({
                    "type": "range", "min": 2.5, "max": -1.5,
                    "min_inclusive": false, "max_inclusive": true
                }),
                NumberConstraint::range(-1.5, 2.5, true, false),
            ),
        ] {
            let reader =
                serde_json::from_value::<RootObject<ConstraintNumberDefinitionReaderV1>>(value)
                    .unwrap();
            assert_eq!(NumberConstraint::from(reader.document), expected);
        }
    }

    #[test]
    fn rejects_invalid_tags_missing_bounds_and_unknown_fields() {
        for value in [
            json!({"type": 1}),
            json!({"type": "unknown"}),
            json!({"type": "min", "min": 1}),
            json!({"type": "none", "min": 1}),
        ] {
            assert!(
                serde_json::from_value::<RootObject<ConstraintIntDefinitionReaderV1>>(
                    value.clone()
                )
                .is_err(),
                "{value}"
            );
            assert!(
                serde_json::from_value::<RootObject<ConstraintNumberDefinitionReaderV1>>(value)
                    .is_err()
            );
        }
    }
}
