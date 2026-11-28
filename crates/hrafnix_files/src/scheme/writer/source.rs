use crate::common::serialize::serialize_shareable_string;
use crate::data::source::{ComponentSource, PackageReference, PackageVersion};
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Current JSON representation of an unresolved source reference.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum ComponentSourceWriter {
    /// File-defined component provenance.
    File {
        /// Package omitted for loose component files.
        #[serde(skip_serializing_if = "Option::is_none")]
        package: Option<PackageReferenceWriter>,
        /// Unresolved component path.
        #[serde(serialize_with = "serialize_shareable_string")]
        path: ShareableString,
    },
    /// Built-in component provenance.
    Builtin {
        /// Built-in name.
        #[serde(serialize_with = "serialize_shareable_string")]
        name: ShareableString,
        /// Built-in version.
        version: u32,
    },
}

impl From<&ComponentSource> for ComponentSourceWriter {
    fn from(source: &ComponentSource) -> Self {
        match source {
            ComponentSource::File { package, path } => Self::File {
                package: package.as_ref().map(Into::into),
                path: path.as_str().into(),
            },
            ComponentSource::Builtin { name, version } => Self::Builtin {
                name: name.clone(),
                version: *version,
            },
        }
    }
}

/// Current JSON representation of a package name and version.
#[derive(Debug, Serialize)]
pub(crate) struct PackageReferenceWriter {
    /// Unresolved package name.
    #[serde(serialize_with = "serialize_shareable_string")]
    name: ShareableString,
    /// Package version.
    version: PackageVersionWriter,
}

impl From<&PackageReference> for PackageReferenceWriter {
    fn from(package: &PackageReference) -> Self {
        Self {
            name: package.name().clone(),
            version: package.version().into(),
        }
    }
}

/// Current JSON representation of a package version.
#[derive(Debug, Serialize)]
pub(crate) struct PackageVersionWriter {
    /// Major version component.
    major: u16,
    /// Minor version component.
    minor: u16,
    /// Patch version component.
    patch: u16,
}

impl From<PackageVersion> for PackageVersionWriter {
    fn from(version: PackageVersion) -> Self {
        Self {
            major: version.major(),
            minor: version.minor(),
            patch: version.patch(),
        }
    }
}
