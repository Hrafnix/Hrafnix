use crate::common::serialize::{serialize_component_key, serialize_shareable_string};
use crate::component_instance::data::{ComponentInstance, ComponentSource, PackageReference};
use crate::frozen_parameter::writer::ParameterFrozenWriter;
use crate::graphics::writer::PositionWriter;
use crate::metadata_gui::writer::GuiMetadataWriter;
use hrafnix_keys::ComponentKey;
use hrafnix_shareable_string::ShareableString;
use serde::{Serialize, Serializer, ser::Error};
use std::collections::BTreeSet;

/// Current JSON representation of a saved component instance.
#[derive(Debug, Serialize)]
pub(crate) struct ComponentInstanceWriter {
    /// Plain-string component identifier.
    #[serde(serialize_with = "serialize_component_key")]
    id: ComponentKey,
    /// Tagged source reference.
    source: ComponentSourceWriter,
    /// Position in the containing drawing.
    position: PositionWriter,
    /// Saved dimensions, icon, and bounded port locations.
    gui_metadata: GuiMetadataWriter,
    /// Ordered parameter definitions and expressions.
    parameters: ParameterFrozenWriter,
}

impl From<&ComponentInstance> for ComponentInstanceWriter {
    fn from(instance: &ComponentInstance) -> Self {
        Self {
            id: instance.id.clone(),
            source: ComponentSourceWriter::from(&instance.source),
            position: PositionWriter::from(&instance.position),
            gui_metadata: GuiMetadataWriter::from(&instance.gui_metadata),
            parameters: ParameterFrozenWriter::from(&instance.parameters),
        }
    }
}

/// Serializes instances without emitting duplicate component identifiers.
pub(crate) fn serialize_component_instances<S: Serializer>(
    instances: &[ComponentInstanceWriter],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut ids = BTreeSet::new();
    for instance in instances {
        if !ids.insert(&instance.id) {
            return Err(S::Error::custom(format!(
                "duplicate component id `{}`",
                instance.id
            )));
        }
    }
    instances.serialize(serializer)
}

/// Current JSON representation of an unresolved source reference.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ComponentSourceWriter {
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
struct PackageReferenceWriter {
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
