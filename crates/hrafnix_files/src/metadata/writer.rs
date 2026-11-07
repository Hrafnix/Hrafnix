use crate::common::serialize::serialize_shareable_string;
use crate::metadata::data::MetadataFile;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// The current JSON representation of the `metadata` entry used when writing.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct MetadataWriter {
    /// Name shown to users.
    #[serde(serialize_with = "serialize_shareable_string")]
    display_name: ShareableString,
    /// Free-form description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Author of the file.
    #[serde(serialize_with = "serialize_shareable_string")]
    author: ShareableString,
    /// Major part of the minimum Hrafnix version.
    minimum_version_major: u16,
    /// Minor part of the minimum Hrafnix version.
    minimum_version_minor: u16,
}

impl From<&MetadataFile> for MetadataWriter {
    fn from(metadata: &MetadataFile) -> Self {
        Self {
            display_name: metadata.display_name.clone(),
            description: metadata.description.clone(),
            author: metadata.author.clone(),
            minimum_version_major: metadata.minimum_version_major,
            minimum_version_minor: metadata.minimum_version_minor,
        }
    }
}
