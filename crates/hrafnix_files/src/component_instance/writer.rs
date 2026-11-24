use crate::common::serialize::serialize_component_key;
use crate::common::validate::ensure_unique_keys;
use crate::component_instance::data::ComponentInstance;
use crate::frozen_parameter::writer::ParameterFrozenWriter;
use crate::graphics::writer::PositionWriter;
use crate::metadata_gui::writer::GuiMetadataWriter;
use crate::source::writer::ComponentSourceWriter;
use hrafnix_keys::ComponentKey;
use hrafnix_message::message::Message;
use serde::Serialize;

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

/// Rejects instances that would be saved with repeated component identifiers.
pub(crate) fn ensure_unique_component_ids(instances: &[ComponentInstance]) -> Result<(), Message> {
    ensure_unique_keys(
        instances.iter().map(|instance| &instance.id),
        "files_duplicate_component_id",
    )
}
