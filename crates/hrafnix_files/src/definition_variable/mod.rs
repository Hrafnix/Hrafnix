//! Ordered variable-definition codecs for model and component files.
//!
//! File data uses [`hrafnix_datastore::definition::VariableObjectDefinition`]
//! directly; these codecs preserve item order, descriptions, and defaults.

/// Version-specific variable-definition readers.
pub(crate) mod reader;
/// Current variable-definition writer.
pub(crate) mod writer;
