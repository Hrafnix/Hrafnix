use crate::common::deserialize::deserialize_object;
use crate::common::validate::ensure_unique_keys;
use crate::data::component_instance::ComponentInstance;
use crate::scheme::reader::v1::frozen_parameter::ParameterFrozenReaderV1;
use crate::scheme::reader::v1::graphics::PositionReaderV1;
use crate::scheme::reader::v1::metadata_gui::GuiMetadataReaderV1;
use crate::scheme::reader::v1::source::ComponentSourceReaderV1;
use hrafnix_keys::ComponentKey;
use hrafnix_message::message::Message;
use serde::Deserialize;

/// Version 1 component instance with its saved GUI and parameter snapshot.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ComponentInstanceReaderV1 {
    /// Plain-string component identifier, validated during conversion.
    id: String,
    /// Tagged reference to a file or built-in definition.
    #[serde(deserialize_with = "deserialize_object")]
    source: ComponentSourceReaderV1,
    /// Position in the containing drawing.
    #[serde(deserialize_with = "deserialize_object")]
    position: PositionReaderV1,
    /// Saved dimensions, rotation, icon path, and ports.
    #[serde(deserialize_with = "deserialize_object")]
    gui_metadata: GuiMetadataReaderV1,
    /// Ordered frozen parameters with embedded definitions.
    parameters: ParameterFrozenReaderV1,
}

impl TryFrom<ComponentInstanceReaderV1> for ComponentInstance {
    type Error = Message;

    fn try_from(instance: ComponentInstanceReaderV1) -> Result<Self, Self::Error> {
        Ok(Self::new(
            ComponentKey::new(instance.id.into())?,
            instance.source.into(),
            instance.position.into(),
            instance.gui_metadata.try_into()?,
            instance.parameters.try_into()?,
        ))
    }
}

/// Converts instances in file order, rejecting repeated component identifiers.
pub(crate) fn convert_component_instances(
    instances: Vec<ComponentInstanceReaderV1>,
) -> Result<Vec<ComponentInstance>, Message> {
    ensure_unique_keys(
        instances.iter().map(|instance| &instance.id),
        "files_duplicate_component_id",
    )?;
    instances.into_iter().map(TryInto::try_into).collect()
}
