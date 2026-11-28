use crate::common::deserialize::{
    StringTagDeserializer, deserialize_object, deserialize_optional_object,
};
use crate::data::source::{ComponentSource, PackageReference, PackageVersion};
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
        version: u32,
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
                version,
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
    #[serde(deserialize_with = "deserialize_object")]
    version: PackageVersionReaderV1,
}

impl From<PackageReferenceReaderV1> for PackageReference {
    fn from(package: PackageReferenceReaderV1) -> Self {
        Self::new(package.name.into(), package.version.into())
    }
}

/// Version 1 package version with required major, minor, and patch integers.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PackageVersionReaderV1 {
    /// Major version component.
    major: u16,
    /// Minor version component.
    minor: u16,
    /// Patch version component.
    patch: u16,
}

impl From<PackageVersionReaderV1> for PackageVersion {
    fn from(version: PackageVersionReaderV1) -> Self {
        Self::new(version.major, version.minor, version.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentSourceReaderV1;
    use crate::common::deserialize::RootObject;
    use crate::data::source::{ComponentSource, PackageReference, PackageVersion};
    use serde_json::json;

    #[test]
    fn reads_loose_packaged_and_builtin_sources_without_resolution() {
        for (value, expected) in [
            (
                json!({"type": "file", "path": "../missing.hxc"}),
                ComponentSource::File {
                    package: None,
                    path: "../missing.hxc".into(),
                },
            ),
            (
                json!({"type": "file", "package": null, "path": ""}),
                ComponentSource::File {
                    package: None,
                    path: "".into(),
                },
            ),
            (
                json!({
                    "type": "file", "path": "components/example.hxc",
                    "package": {"name": "example.package", "version": {"major": 1, "minor": 2, "patch": 65535}},
                }),
                ComponentSource::File {
                    package: Some(PackageReference::new(
                        "example.package".into(),
                        PackageVersion::new(1, 2, u16::MAX),
                    )),
                    path: "components/example.hxc".into(),
                },
            ),
            (
                json!({"type": "builtin", "name": "builtin.example", "version": u32::MAX}),
                ComponentSource::Builtin {
                    name: "builtin.example".into(),
                    version: u32::MAX,
                },
            ),
        ] {
            let reader: RootObject<ComponentSourceReaderV1> =
                serde_json::from_value(value).unwrap();
            assert_eq!(ComponentSource::from(reader.document), expected);
        }
    }

    #[test]
    fn rejects_invalid_tags_fields_and_versions() {
        for value in [
            json!({"type": 0, "path": "example.hxc"}),
            json!({"type": "unknown", "path": "example.hxc"}),
            json!({"type": "file"}),
            json!({"type": "file", "path": "", "extra": 1}),
            json!({"type": "builtin", "name": "example", "version": -1}),
            json!({"type": "builtin", "name": "example", "version": 4294967296_u64}),
            json!({"type": "file", "path": "", "package": {"name": "example", "version": [1, 2, 3]}}),
            json!({"type": "file", "path": "", "package": {"name": "example", "version": {"major": 1, "minor": 2}}}),
            json!({"type": "file", "path": "", "package": {"name": "example", "version": {"major": 65536, "minor": 2, "patch": 3}}}),
        ] {
            assert!(
                serde_json::from_value::<RootObject<ComponentSourceReaderV1>>(value.clone())
                    .is_err(),
                "{value}"
            );
        }
    }
}
