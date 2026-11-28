use crate::common::serialize::serialize_component_key;
use crate::common::validate::ensure_unique_keys;
use crate::data::component_instance::ComponentInstance;
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

#[cfg(test)]
mod tests {
    use super::{ComponentInstanceWriter, ensure_unique_component_ids};
    use crate::data::component_instance::ComponentInstance;
    use crate::data::graphics::{Dimension, Position, Rotation};
    use crate::data::metadata_gui::GuiMetadata;
    use crate::data::source::ComponentSource;
    use hrafnix_datastore::definition::ParameterObjectDefinition;
    use hrafnix_datastore::frozen::ParameterObjectFrozen;
    use hrafnix_keys::{ComponentKey, component_key};
    use serde_json::json;

    /// Minimal instance data without nested parameter or port definitions.
    fn instance(id: ComponentKey) -> ComponentInstance {
        ComponentInstance::new(
            id,
            ComponentSource::Builtin {
                name: "builtin.example".into(),
                version: 1,
            },
            Position::new(-10, 20),
            GuiMetadata::new(
                Dimension::new(10, 20),
                Rotation::Degrees90,
                "icon.svg".into(),
                Vec::new(),
            ),
            ParameterObjectFrozen::new(ParameterObjectDefinition::builder().finish()),
        )
    }

    #[test]
    fn writes_all_snapshot_fields_without_interface_hash() {
        let instance = instance(component_key!("example").into());
        assert_eq!(
            serde_json::to_value(ComponentInstanceWriter::from(&instance)).unwrap(),
            json!({
                "id": "example",
                "source": {"type": "builtin", "name": "builtin.example", "version": 1},
                "position": {"x": -10, "y": 20},
                "gui_metadata": {
                    "dimensions": {"width": 10, "height": 20},
                    "rotation": 90, "icon": "icon.svg", "ports": []
                },
                "parameters": []
            }),
        );
    }

    #[test]
    fn validates_component_ids_before_saving() {
        assert!(ensure_unique_component_ids(&[]).is_ok());
        let first = instance(component_key!("example").into());
        let other = instance(component_key!("another").into());
        assert!(ensure_unique_component_ids(&[first.clone(), other]).is_ok());
        let error = ensure_unique_component_ids(&[first.clone(), first]).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_duplicate_component_id"
        );
    }
}
