//! Versioned JSON readers and current-format writers, separate from file data.

/// File-version dispatch and versioned deserialization.
pub mod reader;
/// Current-format file and nested-value serialization.
pub mod writer;
