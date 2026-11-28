use crate::common::serialize::serialize_component_key;
use crate::common::validate::ensure_unique_keys;
use crate::component_instance::data::ComponentInstance;
use crate::scheme::writer::frozen_parameter::ParameterFrozenWriter;
use crate::scheme::writer::graphics::PositionWriter;
use crate::scheme::writer::metadata_gui::GuiMetadataWriter;
use crate::scheme::writer::source::ComponentSourceWriter;
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
    /// Saved dimensions, rotation, icon, and bounded port locations.
    gui_metadata: GuiMetadataWriter,
    /// Ordered parameter definitions and expressions.
    parameters: ParameterFrozenWriter,
}

impl From<&ComponentInstance> for ComponentInstanceWriter {
    fn from(instance: &ComponentInstance) -> Self {
        let position = instance.position();
        Self {
            id: instance.id().clone(),
            source: ComponentSourceWriter::from(instance.source()),
            position: PositionWriter::from(&position),
            gui_metadata: GuiMetadataWriter::from(instance.gui_metadata()),
            parameters: ParameterFrozenWriter::from(instance.parameters()),
        }
    }
}

/// Rejects instances that would be saved with repeated component identifiers.
pub(crate) fn ensure_unique_component_ids(instances: &[ComponentInstance]) -> Result<(), Message> {
    ensure_unique_keys(
        instances.iter().map(ComponentInstance::id),
        "files_duplicate_component_id",
    )
}
