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
    pub name: ShareableString,
    /// Package version.
    pub version: ShareableString,
}
