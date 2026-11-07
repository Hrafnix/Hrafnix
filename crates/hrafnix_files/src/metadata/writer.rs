use crate::metadata::data::Metadata;
use serde::Serialize;

/// The current JSON representation of the `metadata` entry used when writing.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct MetadataDocument {
    /// Name shown to users.
    display_name: String,
    /// Free-form description.
    description: String,
    /// Author of the file.
    author: String,
    /// Major part of the minimum Hrafnix version.
    minimum_version_major: u16,
    /// Minor part of the minimum Hrafnix version.
    minimum_version_minor: u16,
}

impl From<&Metadata> for MetadataDocument {
    fn from(metadata: &Metadata) -> Self {
        Self {
            display_name: metadata.display_name.to_string(),
            description: metadata.description.to_string(),
            author: metadata.author.to_string(),
            minimum_version_major: metadata.minimum_version_major,
            minimum_version_minor: metadata.minimum_version_minor,
        }
    }
}