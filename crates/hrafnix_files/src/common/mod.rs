/// Shared Serde deserialization helpers.
pub(crate) mod deserialize;
/// Localized file and JSON diagnostics.
pub(crate) mod error;
/// Bounded UTF-8 file reads and atomic writes.
pub(crate) mod file;
/// Shared Serde serialization helpers.
pub(crate) mod serialize;
/// Test helpers that read documents through parsing and conversion.
#[cfg(test)]
pub(crate) mod test_support;
/// Localized post-parse validation shared by readers and writers.
pub(crate) mod validate;
