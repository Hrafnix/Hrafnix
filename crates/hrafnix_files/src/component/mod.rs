use crate::common::error::{json_error, json_serialization_error};
use crate::common::file::{read_file_to_string, write_string_to_file};
use crate::component::data::ComponentFile;
use crate::component::reader::versions::ComponentReader;
use crate::component::writer::ComponentWriter;
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
/// contains invalid UTF-8, or does not match a supported component JSON format.
pub fn read_component_file(path: &Utf8Path) -> Result<ComponentFile, Message> {
    let component_string = read_file_to_string(path)?;

    let component_deserialized: ComponentReader =
        serde_json::from_str(&component_string).map_err(|error| json_error(path, error))?;
    let component_file: ComponentFile = component_deserialized.into();
    Ok(component_file)
}

/// Writes a component file as pretty-printed JSON without a final newline.
///
/// # Errors
///
/// Returns a localized error if serialization or atomic file replacement fails.
pub fn write_component_file(component_file: ComponentFile, path: &Utf8Path) -> Result<(), Message> {
    let component_writer: ComponentWriter = component_file.into();

    let component_serialized: String = serde_json::to_string_pretty(&component_writer)
        .map_err(|error| json_serialization_error(path, error))?;
    write_string_to_file(component_serialized.as_str(), path)?;
    Ok(())
}
