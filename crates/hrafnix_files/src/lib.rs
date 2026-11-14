//! Project, model, and component files.
//!
//! Model and component documents use a string `"file-version"` tag and a named
//! `"model"` or `"component"` object containing metadata. Version 1 is currently
//! supported. Readers accept a leading UTF-8 BOM and reject unknown fields.
//! Documents and their struct contents must be JSON objects; positional arrays
//! are rejected.
//! Model and component contents require `foreground_decorations` and
//! `background_decorations` arrays, including when those arrays are empty.
//! Model and component contents also require a `connections` array, including
//! when empty. Each connection has plain-string `from_component`, `from_port`,
//! `to_component`, and `to_port` identifiers validated as component and port
//! keys, and a `line` object with a required `midpoints` array. Midpoints use
//! signed 32-bit integer `x` and `y` drawing coordinates and retain their order.
//! Connections have no style field. Endpoint references are stored without
//! resolving the referenced components or ports.
//! Component contents also require a `gui_metadata` object with `dimensions`
//! (`width` and `height` as unsigned 16-bit integers) and an `icon` path string.
//! Icon paths are stored unchanged, without opening or resolving them.
//! GUI metadata also requires a `ports` array, including when empty. Each port
//! has a plain-string `id` validated as a [`hrafnix_keys::PortKey`], a plain-string
//! `type`, and a `location` object with unsigned
//! 16-bit `x` and `y` drawing coordinates. Readers and writers cap `x` to the
//! declared icon width and `y` to its height.
//! Model and component contents require a `translations` object with a source
//! `language` string and a `translations` object mapping original text to target
//! language codes and translated strings. Both fields are required, even for an
//! empty translation table. Duplicate keys and language codes are rejected.
//! Every text decoration property is required and sits directly inside its
//! text object. Background decoration kinds are plain strings such as
//! `"Frame"`. Font sizes are plain integers in hundredths of a drawing unit
//! (`1234` means `12.34`); decimal and exponent values are rejected.
//!
//! Decoration structs expose public fields, while the [`graphics`] value types
//! (positions, dimensions, and font sizes) use constructors and accessors.
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

/// Shapes drawn behind model and component contents.
pub mod background_decoration;
/// Shared file I/O and localized diagnostics.
mod common;
/// Component definitions and behavior.
pub mod component;
/// Connections between component ports and their drawing paths.
pub mod connection;
/// Text and images drawn in front of model and component contents.
pub mod foreground_decoration;
/// Geometry and styling values shared by graphical elements.
pub mod graphics;
/// Drawing dimensions, icons, and port locations associated with components.
pub mod gui_metadata;
/// Metadata shared by model and component files.
pub mod metadata;
/// Simulation model definitions.
pub mod model;
/// Source languages and translated text shared by model and component files.
pub mod translations;
