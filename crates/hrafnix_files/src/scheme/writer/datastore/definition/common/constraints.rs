use hrafnix_datastore::definition::{IntegerConstraintEnum, NumberConstraintEnum};
use serde::Serialize;

/// Version-1 integer bounds.
#[derive(Debug, Serialize)]
#[serde(
    remote = "IntegerConstraintEnum",
    tag = "type",
    rename_all = "snake_case"
)]
pub(crate) enum ConstraintIntDefinitionWriterV1 {
    /// No bounds.
    None,
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

/// Version-1 numeric bounds.
#[derive(Debug, Serialize)]
#[serde(
    remote = "NumberConstraintEnum",
    tag = "type",
    rename_all = "snake_case"
)]
pub(crate) enum ConstraintNumberDefinitionWriterV1 {
    /// No bounds.
    None,
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, value::Serializer};

    #[test]
    fn writes_every_integer_constraint_with_exact_fields() {
        for (constraint, expected) in [
            (IntegerConstraintEnum::None, json!({"type": "none"})),
            (
                IntegerConstraintEnum::Min {
                    min: -3,
                    inclusive: false,
                },
                json!({"type": "min", "min": -3, "inclusive": false}),
            ),
            (
                IntegerConstraintEnum::Max {
                    max: 8,
                    inclusive: true,
                },
                json!({"type": "max", "max": 8, "inclusive": true}),
            ),
            (
                IntegerConstraintEnum::Range {
                    min: -3,
                    max: 8,
                    min_inclusive: false,
                    max_inclusive: true,
                },
                json!({
                    "type": "range", "min": -3, "max": 8,
                    "min_inclusive": false, "max_inclusive": true
                }),
            ),
        ] {
            assert_eq!(
                ConstraintIntDefinitionWriterV1::serialize(&constraint, Serializer).unwrap(),
                expected
            );
        }
    }

    #[test]
    fn writes_every_number_constraint_with_exact_fields() {
        for (constraint, expected) in [
            (NumberConstraintEnum::None, json!({"type": "none"})),
            (
                NumberConstraintEnum::Min {
                    min: -1.5,
                    inclusive: false,
                },
                json!({"type": "min", "min": -1.5, "inclusive": false}),
            ),
            (
                NumberConstraintEnum::Max {
                    max: 2.5,
                    inclusive: true,
                },
                json!({"type": "max", "max": 2.5, "inclusive": true}),
            ),
            (
                NumberConstraintEnum::Range {
                    min: -1.5,
                    max: 2.5,
                    min_inclusive: false,
                    max_inclusive: true,
                },
                json!({
                    "type": "range", "min": -1.5, "max": 2.5,
                    "min_inclusive": false, "max_inclusive": true
                }),
            ),
        ] {
            assert_eq!(
                ConstraintNumberDefinitionWriterV1::serialize(&constraint, Serializer).unwrap(),
                expected
            );
        }
    }
}
