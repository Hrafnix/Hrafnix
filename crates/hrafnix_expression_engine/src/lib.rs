//! Expression engine crate.

// Test code favors clarity and brevity over the strictness we require of library code:
// panicking helpers (`unwrap`/`expect`/indexing/`panic!`) and approximate float comparisons
// are idiomatic and expected in tests.
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::panic,
        clippy::float_cmp,
        clippy::as_conversions,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::unreadable_literal,
        clippy::unnecessary_wraps,
        clippy::similar_names,
        clippy::arithmetic_side_effects,
        clippy::wildcard_enum_match_arm
    )
)]

use hrafnix_datastore::definition::{
    BooleanDefinition, ChoiceDefinition, FileDefinition, FolderDefinition, IntegerDefinition,
    NumberDefinition, NumberWithUnitsDefinition, StringDefinition, UnitDefinition,
};
use hrafnix_shareable_string::SharedStringStore;

/// Processed data.
pub mod computed_data;
/// Evaluation engine.
pub mod evaluation;
/// Input data.
pub mod input_data;
/// Convenience re-exports for common types.
pub mod prelude;

pub use computed_data::*;
pub use evaluation::*;
pub use input_data::*;

/// A definition for one of the basic (non-composite) data types supported by the
/// expression engine.
#[derive(Debug, Clone, PartialEq)]
pub enum BasicDefinition {
    /// Holds a boolean value.
    Boolean(BooleanDefinition),
    /// Holds a value from a fixed set of choices.
    Choice(ChoiceDefinition),
    /// Holds a file reference.
    File(FileDefinition),
    /// Holds a folder reference.
    Folder(FolderDefinition),
    /// Holds an integer value.
    Integer(IntegerDefinition),
    /// Holds numeric value.
    Number(NumberDefinition),
    /// Holds a numeric value with associated units.
    NumberWithUnits(NumberWithUnitsDefinition),
    /// Holds a string value.
    String(StringDefinition),
    /// Holds a unit value.
    Unit(UnitDefinition),
}

impl BasicDefinition {
    /// Interns this value's strings in `store` in place, preserving its contents.
    pub fn launder(&mut self, store: &SharedStringStore) {
        match self {
            Self::Boolean(value) => value.launder(store),
            Self::Choice(value) => value.launder(store),
            Self::File(value) => value.launder(store),
            Self::Folder(value) => value.launder(store),
            Self::Integer(value) => value.launder(store),
            Self::Number(value) => value.launder(store),
            Self::NumberWithUnits(value) => value.launder(store),
            Self::String(value) => value.launder(store),
            Self::Unit(value) => value.launder(store),
        }
    }
}
