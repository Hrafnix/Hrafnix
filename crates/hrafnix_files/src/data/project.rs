use hrafnix_shareable_string::{ShareableString, SharedStringStore};

/// Metadata stored in a project's `hrafnix.toml` manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectFile {
    /// User-visible project name.
    name: ShareableString,
    /// Project version, stored without semantic-version validation.
    version: ShareableString,
}

impl ProjectFile {
    /// Creates project manifest metadata.
    #[must_use]
    pub(crate) const fn new(name: ShareableString, version: ShareableString) -> Self {
        Self { name, version }
    }

    /// Returns the project name.
    #[must_use]
    pub const fn name(&self) -> &ShareableString {
        &self.name
    }

    /// Returns the project version.
    #[must_use]
    pub const fn version(&self) -> &ShareableString {
        &self.version
    }

    /// Interns the manifest's strings in `store` in place.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.name = store.launder(&self.name);
        self.version = store.launder(&self.version);
    }
}
