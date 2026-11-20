//! Ordered frozen parameter codecs with embedded item definitions and values.
//!
//! File data uses [`hrafnix_datastore::frozen::ParameterObjectFrozen`] directly.
//! Each array item has a plain-string `key` with a `p_` prefix, a snake-case
//! `type`, and the definition and value fields of its shared frozen item codec.
//! Duplicate keys are rejected and array order is preserved.

#![allow(
    dead_code,
    reason = "Frozen object codecs are pending higher-level reader and writer wiring"
)]

/// Version-specific frozen parameter readers.
pub(crate) mod reader;
/// Current frozen parameter writer.
pub(crate) mod writer;
