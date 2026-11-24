use crate::common::deserialize::{StringTagDeserializer, deserialize_optional_object};
use crate::source::data::{ComponentSource, PackageReference};
use serde::{Deserialize, Deserializer};

/// Version 1 source reference selected by a plain-string type tag.
#[derive(Debug, Deserialize)]
#[serde(
    remote = "Self",
    tag = "type",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub(crate) enum ComponentSourceReaderV1 {
    /// Unresolved file reference.
    File {
        /// Optional object containing the package name and version.
        #[serde(default, deserialize_with = "deserialize_optional_object")]
        package: Option<PackageReferenceReaderV1>,
        /// Unresolved component path.
        path: String,
    },
    /// Unresolved built-in reference.
    Builtin {
        /// Built-in name.
        name: String,
        /// Built-in version.
        version: String,
    },
}

impl<'de> Deserialize<'de> for ComponentSourceReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::deserialize(StringTagDeserializer(deserializer))
    }
}

impl From<ComponentSourceReaderV1> for ComponentSource {
    fn from(source: ComponentSourceReaderV1) -> Self {
        match source {
            ComponentSourceReaderV1::File { package, path } => Self::File {
                package: package.map(Into::into),
                path: path.into(),
            },
            ComponentSourceReaderV1::Builtin { name, version } => Self::Builtin {
                name: name.into(),
                version: version.into(),
            },
        }
    }
}

/// Version 1 package reference with a required name and version.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PackageReferenceReaderV1 {
    /// Unresolved package name.
    name: String,
    /// Package version.
    version: String,
}

impl From<PackageReferenceReaderV1> for PackageReference {
    fn from(package: PackageReferenceReaderV1) -> Self {
        Self {
            name: package.name.into(),
            version: package.version.into(),
        }
    }
}
