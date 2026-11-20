//! Ordered frozen global codecs with embedded item definitions and values.
//!
//! File data uses [`hrafnix_datastore::frozen::GlobalObjectFrozen`] directly.
//! Each array item has a plain-string `key` with a `g_` prefix, a snake-case
//! `type`, and the definition and value fields of its shared frozen item codec.
//! Duplicate keys are rejected and array order is preserved.

/// Version-specific frozen global readers.
pub(crate) mod reader;
/// Current frozen global writer.
pub(crate) mod writer;
