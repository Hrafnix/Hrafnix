use serde::Deserialize;

/// The version 1 JSON representation of the `metadata` entry, separate from the
/// application's metadata.
///
/// Every field is required and unknown fields are rejected so nothing is silently
/// dropped when the file is written back.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(crate) struct Metadata {
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