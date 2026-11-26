use camino::Utf8PathBuf;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};

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
        version: u32,
    },
}

impl ComponentSource {
    /// Returns a copy whose strings are interned in `store`.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        match self {
            Self::File { package, path } => Self::File {
                package: package.as_ref().map(|package| package.launder(store)),
                path: path.clone(),
            },
            Self::Builtin { name, version } => Self::Builtin {
                name: store.launder(name),
                version: *version,
            },
        }
    }
}

/// A semantic package version with major, minor, and patch components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageVersion {
    /// Major version component.
    major: u16,
    /// Minor version component.
    minor: u16,
    /// Patch version component.
    patch: u16,
}

impl PackageVersion {
    /// Creates a new package version with the given major, minor, and patch numbers.
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version component.
    #[must_use]
    pub const fn major(&self) -> u16 {
        self.major
    }

    /// Returns the minor version component.
    #[must_use]
    pub const fn minor(&self) -> u16 {
        self.minor
    }

    /// Returns the patch version component.
    #[must_use]
    pub const fn patch(&self) -> u16 {
        self.patch
    }
}

/// An unresolved reference to a specific package version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageReference {
    /// Package name, such as `hrafnix.sensors`.
    name: ShareableString,
    /// Package version.
    version: PackageVersion,
}

impl PackageReference {
    /// Creates a new package reference with the given name and version.
    #[must_use]
    pub const fn new(name: ShareableString, version: PackageVersion) -> Self {
        Self { name, version }
    }

    /// Returns the package name.
    #[must_use]
    pub const fn name(&self) -> &ShareableString {
        &self.name
    }

    /// Returns the package version.
    #[must_use]
    pub const fn version(&self) -> PackageVersion {
        self.version
    }

    /// Returns a copy whose strings are interned in `store`.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            name: store.launder(&self.name),
            version: self.version,
        }
    }
}
