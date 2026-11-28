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
