use crate::common::deserialize::{
    StringTagDeserializer, deserialize_component_key, deserialize_object, deserialize_objects,
    deserialize_optional_object,
};
use crate::component_instance::data::{ComponentInstance, ComponentSource, PackageReference};
use crate::frozen_parameter::reader::v1::ParameterFrozenReaderV1;
use crate::graphics::reader::v1::PositionReaderV1;
use crate::metadata_gui::reader::v1::GuiMetadataReaderV1;
use hrafnix_keys::ComponentKey;
use hrafnix_message::message::Message;
use serde::{Deserialize, Deserializer, de::Error};
use std::collections::BTreeSet;

/// Version 1 component instance with its saved GUI and parameter snapshot.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ComponentInstanceReaderV1 {
    /// Plain-string component identifier.
    #[serde(deserialize_with = "deserialize_component_key")]
    id: ComponentKey,
    /// Tagged reference to a file or built-in definition.
    #[serde(deserialize_with = "deserialize_object")]
    source: ComponentSourceReaderV1,
    /// Position in the containing drawing.
    #[serde(deserialize_with = "deserialize_object")]
    position: PositionReaderV1,
    /// Saved dimensions, icon path, and ports.
    #[serde(deserialize_with = "deserialize_object")]
    gui_metadata: GuiMetadataReaderV1,
    /// Ordered frozen parameters with embedded definitions.
    parameters: ParameterFrozenReaderV1,
}

impl TryFrom<ComponentInstanceReaderV1> for ComponentInstance {
    type Error = Message;

    fn try_from(instance: ComponentInstanceReaderV1) -> Result<Self, Self::Error> {
        Ok(Self {
            id: instance.id,
            source: instance.source.into(),
            position: instance.position.into(),
            gui_metadata: instance.gui_metadata.into(),
            parameters: instance.parameters.try_into()?,
        })
    }
}

/// Reads an ordered array of component objects with unique identifiers.
pub(crate) fn deserialize_component_instances<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<ComponentInstanceReaderV1>, D::Error> {
    let instances = deserialize_objects::<ComponentInstanceReaderV1, D>(deserializer)?;
    let mut ids = BTreeSet::new();
    for instance in &instances {
        if !ids.insert(&instance.id) {
            return Err(D::Error::custom(format!(
                "duplicate component id `{}`",
                instance.id
            )));
        }
    }
    Ok(instances)
}

/// Version 1 source reference selected by a plain-string type tag.
#[derive(Debug, Deserialize)]
#[serde(
    remote = "Self",
    tag = "type",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum ComponentSourceReaderV1 {
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
struct PackageReferenceReaderV1 {
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
