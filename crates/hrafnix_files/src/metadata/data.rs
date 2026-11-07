use hrafnix_shareable_string::ShareableString;

/// Descriptive information shared by model and component files.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetadataFile {
    /// Name shown to users.
    pub display_name: ShareableString,
    /// Free-form description.
    pub description: ShareableString,
    /// Author of the file.
    pub author: ShareableString,
    /// Major part of the minimum Hrafnix version required to open the file.
    pub minimum_version_major: u16,
    /// Minor part of the minimum Hrafnix version required to open the file.
    pub minimum_version_minor: u16,
}
