use crate::common::serialize::serialize_shareable_string;
use crate::source::data::{ComponentSource, PackageReference};
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
        #[serde(serialize_with = "serialize_shareable_string")]
        version: ShareableString,
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
                version: version.clone(),
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
    #[serde(serialize_with = "serialize_shareable_string")]
    version: ShareableString,
}

impl From<&PackageReference> for PackageReferenceWriter {
    fn from(package: &PackageReference) -> Self {
        Self {
            name: package.name.clone(),
            version: package.version.clone(),
        }
    }
}
