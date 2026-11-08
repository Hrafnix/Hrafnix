use crate::common::error::{json_error, json_serialization_error};
use crate::common::file::{read_file_to_string, write_string_to_file};
use crate::model::data::ModelFile;
use crate::model::reader::versions::ModelReader;
use crate::model::writer::ModelWriter;
use camino::Utf8Path;
use hrafnix_message::message::Message;

/// In-memory model file data.
pub mod data;
/// Versioned model file deserialization.
pub(crate) mod reader;
/// Model file serialization.
pub(crate) mod writer;

/// Reads a model file from disk.
///
/// # Errors
///
/// Returns a localized error if the file cannot be read, exceeds the size limit,
/// contains invalid UTF-8, or does not match a supported model JSON format.
pub fn read_model_file(path: &Utf8Path) -> Result<ModelFile, Message> {
    let model_string = read_file_to_string(path)?;

    let model_deserialized: ModelReader =
        serde_json::from_str(&model_string).map_err(|error| json_error(path, error))?;
    let model_file: ModelFile = model_deserialized.into();
    Ok(model_file)
}

/// Writes a model file as pretty-printed JSON without a final newline.
///
/// # Errors
///
/// Returns a localized error if serialization or atomic file replacement fails.
pub fn write_model_file(model_file: ModelFile, path: &Utf8Path) -> Result<(), Message> {
    let model_writer: ModelWriter = model_file.into();

    let model_serialized: String = serde_json::to_string_pretty(&model_writer)
        .map_err(|error| json_serialization_error(path, error))?;
    write_string_to_file(model_serialized.as_str(), path)?;
    Ok(())
}
