//! Reader selection by the document's `file-version` tag.

/// Version 1 file and nested-value deserialization.
pub mod v1;

/// Component file version dispatch.
pub(crate) mod component;
/// Model file version dispatch.
pub(crate) mod model;
