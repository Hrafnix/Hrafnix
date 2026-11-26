use hrafnix_shareable_string::{ShareableString, SharedStringStore};

/// Descriptive information shared by model and component files.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetadataFile {
    /// Name shown to users.
    display_name: ShareableString,
    /// Free-form description.
    description: ShareableString,
    /// Author of the file.
    author: ShareableString,
    /// Major part of the minimum Hrafnix version required to open the file.
    minimum_version_major: u16,
    /// Minor part of the minimum Hrafnix version required to open the file.
    minimum_version_minor: u16,
}

impl MetadataFile {
    /// Creates new descriptive metadata for a model or component file.
    #[must_use]
    pub(crate) const fn new(
        display_name: ShareableString,
        description: ShareableString,
        author: ShareableString,
        minimum_version_major: u16,
        minimum_version_minor: u16,
    ) -> Self {
        Self {
            display_name,
            description,
            author,
            minimum_version_major,
            minimum_version_minor,
        }
    }

    /// Returns the display name shown to users.
    #[must_use]
    pub const fn display_name(&self) -> &ShareableString {
        &self.display_name
    }

    /// Returns the free-form description.
    #[must_use]
    pub const fn description(&self) -> &ShareableString {
        &self.description
    }

    /// Returns the author of the file.
    #[must_use]
    pub const fn author(&self) -> &ShareableString {
        &self.author
    }

    /// Returns the major part of the minimum Hrafnix version required to open the file.
    #[must_use]
    pub const fn minimum_version_major(&self) -> u16 {
        self.minimum_version_major
    }

    /// Returns the minor part of the minimum Hrafnix version required to open the file.
    #[must_use]
    pub const fn minimum_version_minor(&self) -> u16 {
        self.minimum_version_minor
    }

    /// Returns a copy whose strings are interned in `store`.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            display_name: store.launder(&self.display_name),
            description: store.launder(&self.description),
            author: store.launder(&self.author),
            minimum_version_major: self.minimum_version_major,
            minimum_version_minor: self.minimum_version_minor,
        }
    }
}
