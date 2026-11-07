use serde::Deserialize;
use crate::metadata::reader::v1::Metadata;

/// The version 1 JSON representation of the `model` entry, separate from the
/// application's model data.
///
/// The file version lives beside the entry at the top level, which [`read_document`]
/// handles. Unknown fields are rejected: anything silently ignored here would be
/// dropped when the file is written back, so unexpected content is an error instead.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Model {
    /// Descriptive information about the model as an object with named fields.
    metadata: Metadata,
}