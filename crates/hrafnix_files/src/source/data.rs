use camino::Utf8PathBuf;
use hrafnix_shareable_string::ShareableString;

/// Provenance of a component instance, stored without resolving its source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentSource {
    /// A component definition stored in a file.
    File {
        /// Optional package and its version containing the component file.
        package: Option<PackageReference>,
        /// Component path stored unchanged.
        path: Utf8PathBuf,
    },
    /// A component definition supplied by the application.
    Builtin {
        /// Built-in component name, such as `builtin.clock`.
        name: ShareableString,
        /// Built-in component version.
        version: ShareableString,
    },
}

/// An unresolved reference to a specific package version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageReference {
    /// Package name, such as `hrafnix.sensors`.
    name: ShareableString,
    /// Package version.
    version: ShareableString,
}

impl PackageReference {
    /// Creates a new package reference with the given name and version.
    #[must_use]
    pub const fn new(name: ShareableString, version: ShareableString) -> Self {
        Self { name, version }
    }

    /// Returns the package name.
    #[must_use]
    pub const fn name(&self) -> &ShareableString {
        &self.name
    }

    /// Returns the package version.
    #[must_use]
    pub const fn version(&self) -> &ShareableString {
        &self.version
    }
}
