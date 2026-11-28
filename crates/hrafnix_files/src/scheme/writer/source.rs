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

#[cfg(test)]
mod tests {
    use super::ComponentSourceWriter;
    use crate::data::source::{ComponentSource, PackageReference, PackageVersion};
    use serde_json::json;

    #[test]
    fn writes_loose_files_without_a_package_or_standalone_version() {
        let source = ComponentSource::File {
            package: None,
            path: "../missing.hxc".into(),
        };
        assert_eq!(
            serde_json::to_value(ComponentSourceWriter::from(&source)).unwrap(),
            json!({"type": "file", "path": "../missing.hxc"}),
        );
    }

    #[test]
    fn writes_packaged_files_and_builtins_with_their_distinct_version_shapes() {
        let source = ComponentSource::File {
            package: Some(PackageReference::new(
                "example.package".into(),
                PackageVersion::new(1, 2, u16::MAX),
            )),
            path: "components/example.hxc".into(),
        };
        assert_eq!(
            serde_json::to_value(ComponentSourceWriter::from(&source)).unwrap(),
            json!({
                "type": "file", "path": "components/example.hxc",
                "package": {"name": "example.package", "version": {"major": 1, "minor": 2, "patch": 65535}},
            }),
        );
        let source = ComponentSource::Builtin {
            name: "builtin.example".into(),
            version: u32::MAX,
        };
        assert_eq!(
            serde_json::to_value(ComponentSourceWriter::from(&source)).unwrap(),
            json!({"type": "builtin", "name": "builtin.example", "version": u32::MAX}),
        );
    }
}
