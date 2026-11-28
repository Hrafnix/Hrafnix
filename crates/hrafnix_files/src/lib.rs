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
//! Model and component contents require a `components` array, including when
//! empty. Each instance has a unique plain-string component `id`, a `position`
//! in signed 32-bit drawing coordinates, `gui_metadata`, and an ordered
//! `parameters` array with embedded definitions and frozen expression values.
//! Its `source` is tagged with `"type": "file"` (string `path`
//! and optional `package` object with required string `name` and
//! `version` object containing unsigned 16-bit `major`, `minor`, and `patch`
//! integers) or `"type": "builtin"` (string `name` and unsigned 32-bit `version`). File sources
//! have no standalone version; loose files omit the package. Source versions,
//! names and paths are stored unchanged without resolution or
//! verification. GUI metadata and parameters are saved snapshots, not loaded
//! from the referenced source.
//! Component contents also require a `gui_metadata` object with `dimensions`
//! (`width` and `height` as unsigned 16-bit integers) and an `icon` path string.
//! Icon paths are stored unchanged, without opening or resolving them.
//! GUI metadata requires a `rotation` in unsigned 16-bit integer clockwise degrees
//! about the center of the icon bounds. Readers round to the nearest 90 degrees,
//! with halfway values rounded clockwise and full turns wrapped to zero.
//! Writers emit only 0, 90, 180, or 270. Dimensions and port locations remain
//! unrotated; rotation applies to the icon and ports together and contributes
//! to GUI interface hashes.
//! GUI metadata also requires a `ports` array, including when empty. Each port
//! has a plain-string `id` validated as a [`hrafnix_keys::PortKey`], a plain-string
//! `type`, and a `location` object with unsigned
//! 16-bit `x` and `y` drawing coordinates. Readers and writers cap `x` to the
//! declared icon width and `y` to its height.
//! Model and component contents require a `translations` object with a source
//! `language` string and a `translations` object mapping original text to target
//! language codes and translated strings. Both fields are required, even for an
//! empty translation table. Duplicate keys and language codes are rejected.
//! Model and component contents also require ordered `parameter_definition` and
//! `variable_definition` arrays, including when empty. Items have a plain-string
//! `key` validated as a parameter (`p_`) or variable (`v_`) key, a snake-case
//! `type`, and inline definition fields. Duplicate item keys are rejected.
//! Definitions retain descriptions, constraints, units, and default expressions;
//! these arrays describe the schema rather than storing evaluated values.
//! Model contents also require a `global_settings` array, including when empty.
//! Each item has a unique plain-string `g_` key, a snake-case `type`, an embedded
//! `definition`, and frozen value fields. Global settings preserve definition
//! order, and values must match their embedded definitions.
//! Frozen tables require a `parameter` string (empty for local rows) alongside
//! their `rows`. Unit-aware tables also require a `units` array with exactly one
//! string per defined column. Inside maps, table values are objects with
//! `parameter` and `rows`, plus `units` for unit-aware tables; bare row arrays
//! are not accepted.
//! Every text decoration property is required and sits directly inside its
//! text object. Background decoration kinds are plain strings such as
//! `"Frame"`. Font sizes are plain integers in hundredths of a drawing unit
//! (`1234` means `12.34`); decimal and exponent values are rejected.
//!
//! File-data constructors are crate-private. Callers obtain data through the readers
//! and inspect it through public getter accessors.
//! [`data::source::PackageReference::new`] and [`data::source::PackageVersion::new`]
//! remain public for reader input.
//! String-bearing file-data types expose `launder(&mut self, store)` to intern
//! their strings recursively in place, preserving values, ordering, paths, and hashes.
//!
//! The public read and write functions live in [`formats::model`] and
//! [`formats::component`].
//! JSON readers and writers live under [`scheme`], with version-1 readers grouped
//! in [`scheme::reader::v1`] and current-format writers in [`scheme::writer`].
//! File-version dispatch lives in [`scheme::reader`]; in-memory file-data types
//! live under [`data`].
//! Writers consume the file data and atomically replace the destination with
//! pretty-printed JSON, without a final newline. Reads and writes are limited to
//! 256 MiB.
//! Readers compute `file_hash` from all validated data using field-by-field
//! BLAKE3 encoding, not from JSON bytes. Whitespace, object key order, and a
//! leading BOM do not affect these hashes; ordered arrays retain their order.
//! Component hashes include the loaded source provenance, like `interface_hash`.
//! Hashes are read-time snapshots and are not serialized.

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
/// In-memory model, component, and nested file-data types.
pub mod data;
/// Public file-format reading and writing.
pub mod formats;
/// Versioned JSON schemas for model and component files.
pub mod scheme;
