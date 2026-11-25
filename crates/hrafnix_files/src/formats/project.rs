use crate::common::deserialize::deserialize_object;
use crate::common::error::{toml_error, toml_serialization_error};
use crate::common::file::{read_file_to_string, write_string_to_file, write_string_to_new_file};
use crate::data::project::ProjectFile;
use camino::Utf8Path;
use hrafnix_message::message::Message;
use serde::{Deserialize, Serialize};

/// Strict TOML project document reader.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProjectReader {
    /// Required project metadata table.
    #[serde(deserialize_with = "deserialize_object")]
    project: ProjectMetadataReader,
}

/// Required metadata fields within the project table.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProjectMetadataReader {
    /// Project name as supplied by the manifest.
    name: String,
    /// Project version as supplied by the manifest.
    version: String,
}

/// Borrowed TOML document writer.
#[derive(Debug, Serialize)]
struct ProjectWriter<'a> {
    /// Project metadata table.
    project: ProjectMetadataWriter<'a>,
}

/// Borrowed metadata fields written to the project table.
#[derive(Debug, Serialize)]
struct ProjectMetadataWriter<'a> {
    /// User-visible project name.
    name: &'a str,
    /// Project version string.
    version: &'a str,
}

/// Reads a project manifest from disk.
///
/// # Errors
///
/// Returns a localized error for I/O failures, oversized files, invalid UTF-8,
/// invalid TOML, missing required fields, or unknown fields.
pub fn read_project_file(path: &Utf8Path) -> Result<ProjectFile, Message> {
    let contents = read_file_to_string(path)?;
    let reader: ProjectReader =
        toml::from_str(&contents).map_err(|error| toml_error(path, &error))?;
    Ok(ProjectFile::new(
        reader.project.name.into(),
        reader.project.version.into(),
    ))
}

/// Atomically replaces a project manifest with formatted TOML and a final newline.
///
/// # Errors
///
/// Returns a localized error if serialization or atomic file replacement fails.
pub fn write_project_file(project_file: &ProjectFile, path: &Utf8Path) -> Result<(), Message> {
    let contents = serialize_project_file(project_file, path)?;
    write_string_to_file(&contents, path)
}

/// Atomically creates a project manifest without replacing an existing path.
///
/// The parent directory must already exist. Concurrent creators cannot overwrite
/// one another, and the manifest is only published after its contents are synced.
///
/// # Errors
///
/// Returns a localized error if serialization or file creation fails, including
/// when the destination already exists. Filesystems must support hard links.
pub fn create_project_file(project_file: &ProjectFile, path: &Utf8Path) -> Result<(), Message> {
    let contents = serialize_project_file(project_file, path)?;
    write_string_to_new_file(&contents, path)
}

/// Serializes required project metadata as a TOML table.
fn serialize_project_file(project_file: &ProjectFile, path: &Utf8Path) -> Result<String, Message> {
    let writer = ProjectWriter {
        project: ProjectMetadataWriter {
            name: project_file.name().as_str(),
            version: project_file.version().as_str(),
        },
    };
    toml::to_string_pretty(&writer).map_err(|error| toml_serialization_error(path, &error))
}
