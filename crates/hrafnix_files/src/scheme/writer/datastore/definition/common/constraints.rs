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
