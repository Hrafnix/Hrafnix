//! Ordered parameter-definition codecs for model and component files.
//!
//! File data uses [`hrafnix_datastore::definition::ParameterObjectDefinition`]
//! directly; these codecs preserve item order, descriptions, and defaults.

/// Version-specific parameter-definition readers.
pub(crate) mod reader;
/// Current parameter-definition writer.
pub(crate) mod writer;
