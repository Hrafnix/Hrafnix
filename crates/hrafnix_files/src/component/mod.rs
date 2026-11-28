use crate::common::deserialize::RootObject;
use crate::common::error::{json_error, json_serialization_error};
use crate::common::file::{read_file_to_string, write_string_to_file};
use crate::component::data::ComponentFile;
use crate::scheme::reader::component::ComponentReader;
use crate::scheme::writer::component::ComponentWriter;
use crate::scheme::writer::component_instance::ensure_unique_component_ids;
use crate::source::data::{ComponentSource, PackageReference};
use camino::Utf8Path;
use hrafnix_message::message::Message;

/// In-memory component file data.
pub mod data;

/// Reads a component file from disk with optional package provenance.
///
/// # Errors
///
/// Returns a localized error if the file cannot be read, exceeds the size limit,
/// contains invalid UTF-8, does not match a supported component JSON format, or
/// has instance parameter values that do not match their embedded definitions.
pub fn read_component_file(
    path: &Utf8Path,
    package_information: Option<PackageReference>,
) -> Result<ComponentFile, Message> {
    let component_string = read_file_to_string(path)?;

    let component_deserialized: RootObject<ComponentReader> =
        serde_json::from_str(&component_string).map_err(|error| json_error(path, error))?;
    let source = ComponentSource::File {
        package: package_information,
        path: path.to_owned(),
    };
    component_deserialized
        .document
        .try_into_component_file(source)
}

/// Writes a component file as pretty-printed JSON without a final newline.
///
/// # Errors
///
/// Returns a localized error if serialization or atomic file replacement fails.
pub fn write_component_file(component_file: ComponentFile, path: &Utf8Path) -> Result<(), Message> {
    ensure_unique_component_ids(component_file.components())?;
    let component_writer: ComponentWriter = component_file.into();

    let component_serialized: String = serde_json::to_string_pretty(&component_writer)
        .map_err(|error| json_serialization_error(path, error))?;
    write_string_to_file(component_serialized.as_str(), path)?;
    Ok(())
}
