use crate::common::deserialize::RootObject;
use crate::common::error::{json_error, json_serialization_error};
use crate::common::file::{
    read_file_to_string, read_file_to_string_with_hash, write_string_to_file,
};
use crate::component::data::ComponentFile;
use crate::component::reader::versions::ComponentReader;
use crate::component::writer::ComponentWriter;
use crate::hashes::{FileHashes, component_interface_hash};
use camino::Utf8Path;
use hrafnix_message::message::Message;

/// In-memory component file data.
pub mod data;
/// Versioned component file deserialization.
pub(crate) mod reader;
/// Component file serialization.
pub(crate) mod writer;

/// Reads a component file from disk.
///
/// # Errors
///
/// Returns a localized error if the file cannot be read, exceeds the size limit,
/// contains invalid UTF-8, does not match a supported component JSON format, or
/// has instance parameter values that do not match their embedded definitions.
pub fn read_component_file(path: &Utf8Path) -> Result<ComponentFile, Message> {
    let component_string = read_file_to_string(path)?;
    deserialize_component(&component_string, path)
}

/// Deserializes a component while preserving localized JSON diagnostics.
fn deserialize_component(contents: &str, path: &Utf8Path) -> Result<ComponentFile, Message> {
    let component_deserialized: RootObject<ComponentReader> =
        serde_json::from_str(contents).map_err(|error| json_error(path, error))?;
    component_deserialized.document.try_into()
}

/// Reads a component and returns hashes for the same bytes and parsed interface.
///
/// The whole-file hash includes any leading BOM and original JSON formatting.
///
/// # Errors
///
/// Returns the same errors as [`read_component_file`], or a localized interface
/// serialization error.
pub fn read_component_file_with_hashes(
    path: &Utf8Path,
) -> Result<(ComponentFile, FileHashes), Message> {
    let (contents, whole_file) = read_file_to_string_with_hash(path)?;
    let component = deserialize_component(&contents, path)?;
    let interface = component_interface_hash(&component, path)?;
    Ok((
        component,
        FileHashes {
            whole_file,
            interface,
        },
    ))
}

/// Writes a component file as pretty-printed JSON without a final newline.
///
/// # Errors
///
/// Returns a localized error if serialization or atomic file replacement fails.
pub fn write_component_file(component_file: ComponentFile, path: &Utf8Path) -> Result<(), Message> {
    let component_serialized = serialize_component(component_file, path)?;
    write_string_to_file(&component_serialized, path)
}

/// Writes a component and returns hashes of the bytes and interface saved.
///
/// Hashes are returned only when the existing atomic write procedure succeeds.
/// The destination is not reopened to compute them.
///
/// # Errors
///
/// Returns a localized error if interface/file serialization or atomic file
/// replacement fails.
pub fn write_component_file_with_hashes(
    component_file: ComponentFile,
    path: &Utf8Path,
) -> Result<FileHashes, Message> {
    let interface = component_interface_hash(&component_file, path)?;
    let contents = serialize_component(component_file, path)?;
    let whole_file = blake3::hash(contents.as_bytes());
    write_string_to_file(&contents, path)?;
    Ok(FileHashes {
        whole_file,
        interface,
    })
}

/// Serializes a component in the current pretty-printed file format.
fn serialize_component(component_file: ComponentFile, path: &Utf8Path) -> Result<String, Message> {
    let component_writer: ComponentWriter = component_file.into();

    serde_json::to_string_pretty(&component_writer)
        .map_err(|error| json_serialization_error(path, error))
}
