//! Project, model, and component files.
//!
//! Model and component documents use a string `"file-version"` tag and a named
//! `"model"` or `"component"` object containing metadata. Version 1 is currently
//! supported. Readers accept a leading UTF-8 BOM and reject unknown fields.
//! Documents and their struct contents must be JSON objects; positional arrays
//! are rejected.
//!
//! The public read and write functions live in [`model`] and [`component`].
//! Writers consume the file data and atomically replace the destination with
//! pretty-printed JSON, without a final newline. Reads and writes are limited to
//! 256 MiB.

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
        clippy::arithmetic_side_effects
    )
)]

/// Shared file I/O and localized diagnostics.
mod common;
/// Component definitions and behavior.
pub mod component;
/// Metadata shared by model and component files.
pub mod metadata;
/// Simulation model definitions.
pub mod model;
