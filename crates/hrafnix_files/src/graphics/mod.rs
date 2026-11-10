//! Shared geometry, styling values, and sRGB colors.
//!
//! Colors read as `"theme-default"` or custom `#RRGGBB` / `#RRGGBBAA` strings.
//! Custom colors have alpha last; writers use lowercase hex and omit opaque alpha.
//! The renderer resolves theme defaults according to the color's role.

/// Data structures shared by graphical elements.
pub mod data;
/// Version-specific graphics readers.
pub(crate) mod reader;
/// Current JSON representation of graphics used when writing.
pub mod writer;
