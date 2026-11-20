//! Exact-byte file hashes and deterministic interface fingerprints.
//!
//! Interface version 1 hashes `b"hrafnix/interface/v1\0"` followed by compact
//! JSON containing, in order, `kind`, `gui_metadata`, `parameter_definition`,
//! and `variable_definition`. `kind` is `"component"` or `"model"`;
//! `gui_metadata` is omitted for models. The other fields use the current
//! version-1 file writers, including their normalization and array ordering.
//! Changes to this representation require a new interface hash version.
//!
//! All definition fields participate, including descriptions, defaults,
//! constraints, units, and ordered choices/columns/map items. Component GUI
//! dimensions, icon path, and ordered ports (IDs, types, and capped positions)
//! also participate. Referenced icon/source files are not opened or hashed.
//! Metadata, translations, decorations, nested instances, connections, and
//! model global settings do not participate.
//!
//! Interface equality is exact equality of this normalized public contract,
//! not a guarantee of behavioral or backward compatibility.

use crate::common::error::json_serialization_error;
use crate::component::data::ComponentFile;
use crate::definition_parameter::writer::ParameterDefinitionWriter;
use crate::definition_variable::writer::VariableDefinitionWriter;
use crate::metadata_gui::writer::GuiMetadataWriter;
use crate::model::data::ModelFile;
use camino::Utf8Path;
use hrafnix_message::message::Message;
use serde::Serialize;

/// BLAKE3 fingerprints associated with a successful file read or write.
///
/// Each hash contains 32 bytes, available through [`blake3::Hash::as_bytes`].
/// Hashes are returned to the caller, never embedded in the file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileHashes {
    /// Hash of the exact bytes read or written, including whitespace and any BOM.
    pub whole_file: blake3::Hash,
    /// Version-1 normalized public-interface hash, independent of JSON formatting.
    pub interface: blake3::Hash,
}

/// Computes the version-1 interface hash of an in-memory component.
///
/// Port locations are capped just as they are during file serialization.
/// `path` supplies diagnostic context; this function performs no file I/O.
///
/// # Errors
///
/// Returns a localized error if the interface cannot be serialized.
pub fn component_interface_hash(
    component: &ComponentFile,
    path: &Utf8Path,
) -> Result<blake3::Hash, Message> {
    InterfaceWriter {
        kind: "component",
        gui_metadata: Some(GuiMetadataWriter::from(&component.gui_metadata)),
        parameter_definition: ParameterDefinitionWriter::from(&component.parameter_definition),
        variable_definition: VariableDefinitionWriter::from(&component.variable_definition),
    }
    .hash(path)
}

/// Computes the version-1 interface hash of an in-memory model.
///
/// Only parameter and variable definitions participate.
/// `path` supplies diagnostic context; this function performs no file I/O.
///
/// # Errors
///
/// Returns a localized error if the interface cannot be serialized.
pub fn model_interface_hash(model: &ModelFile, path: &Utf8Path) -> Result<blake3::Hash, Message> {
    InterfaceWriter {
        kind: "model",
        gui_metadata: None,
        parameter_definition: ParameterDefinitionWriter::from(&model.parameter_definition),
        variable_definition: VariableDefinitionWriter::from(&model.variable_definition),
    }
    .hash(path)
}

/// Deterministic version-1 projection of the public file interface.
#[derive(Debug, Serialize)]
struct InterfaceWriter {
    /// Separates model and component fingerprints.
    kind: &'static str,
    /// Component drawing contract; absent for models.
    #[serde(skip_serializing_if = "Option::is_none")]
    gui_metadata: Option<GuiMetadataWriter>,
    /// Complete ordered parameter definitions.
    parameter_definition: ParameterDefinitionWriter,
    /// Complete ordered variable definitions.
    variable_definition: VariableDefinitionWriter,
}

impl InterfaceWriter {
    /// Hashes a versioned domain and the compact, normalized interface JSON.
    fn hash(&self, path: &Utf8Path) -> Result<blake3::Hash, Message> {
        let bytes =
            serde_json::to_vec(self).map_err(|error| json_serialization_error(path, error))?;
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"hrafnix/interface/v1\0");
        hasher.update(&bytes);
        Ok(hasher.finalize())
    }
}
