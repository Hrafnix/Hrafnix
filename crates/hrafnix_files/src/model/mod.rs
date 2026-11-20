use crate::common::deserialize::RootObject;
use crate::common::error::{json_error, json_serialization_error};
use crate::common::file::{
    read_file_to_string, read_file_to_string_with_hash, write_string_to_file,
};
use crate::hashes::{FileHashes, model_interface_hash};
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
/// contains invalid UTF-8, does not match a supported model JSON format, or has
/// global or instance parameter values that do not match their embedded definitions.
pub fn read_model_file(path: &Utf8Path) -> Result<ModelFile, Message> {
    let model_string = read_file_to_string(path)?;
    deserialize_model(&model_string, path)
}

/// Deserializes a model while preserving localized JSON diagnostics.
fn deserialize_model(contents: &str, path: &Utf8Path) -> Result<ModelFile, Message> {
    let model_deserialized: RootObject<ModelReader> =
        serde_json::from_str(contents).map_err(|error| json_error(path, error))?;
    model_deserialized.document.try_into()
}

/// Reads a model and returns hashes for the same bytes and parsed interface.
///
/// The whole-file hash includes any leading BOM and original JSON formatting.
///
/// # Errors
///
/// Returns the same errors as [`read_model_file`], or a localized interface
/// serialization error.
pub fn read_model_file_with_hashes(path: &Utf8Path) -> Result<(ModelFile, FileHashes), Message> {
    let (contents, whole_file) = read_file_to_string_with_hash(path)?;
    let model = deserialize_model(&contents, path)?;
    let interface = model_interface_hash(&model, path)?;
    Ok((
        model,
        FileHashes {
            whole_file,
            interface,
        },
    ))
}

/// Writes a model file as pretty-printed JSON without a final newline.
///
/// # Errors
///
/// Returns a localized error if serialization or atomic file replacement fails.
pub fn write_model_file(model_file: ModelFile, path: &Utf8Path) -> Result<(), Message> {
    let model_serialized = serialize_model(model_file, path)?;
    write_string_to_file(&model_serialized, path)
}

/// Writes a model and returns hashes of the bytes and interface saved.
///
/// Hashes are returned only when the existing atomic write procedure succeeds.
/// The destination is not reopened to compute them.
///
/// # Errors
///
/// Returns a localized error if interface/file serialization or atomic file
/// replacement fails.
pub fn write_model_file_with_hashes(
    model_file: ModelFile,
    path: &Utf8Path,
) -> Result<FileHashes, Message> {
    let interface = model_interface_hash(&model_file, path)?;
    let contents = serialize_model(model_file, path)?;
    let whole_file = blake3::hash(contents.as_bytes());
    write_string_to_file(&contents, path)?;
    Ok(FileHashes {
        whole_file,
        interface,
    })
}

/// Serializes a model in the current pretty-printed file format.
fn serialize_model(model_file: ModelFile, path: &Utf8Path) -> Result<String, Message> {
    let model_writer: ModelWriter = model_file.into();

    serde_json::to_string_pretty(&model_writer)
        .map_err(|error| json_serialization_error(path, error))
}
