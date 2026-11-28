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

#[cfg(test)]
mod tests {
    use super::{ComponentInstanceReaderV1, convert_component_instances};
    use crate::common::deserialize::RootObject;
    use crate::common::test_support::read_value;
    use crate::data::component_instance::ComponentInstance;
    use crate::data::graphics::Position;
    use serde_json::{Value, json};

    /// Minimal instance fields for testing identifier and position conversion.
    fn instance_json() -> Value {
        json!({
            "id": "example",
            "source": {"type": "builtin", "name": "builtin.example", "version": 1},
            "position": {"x": -10, "y": 20},
            "gui_metadata": {
                "dimensions": {"width": 0, "height": 0},
                "rotation": 0, "icon": "", "ports": []
            },
            "parameters": []
        })
    }

    #[test]
    fn converts_identifier_and_position() {
        let instance =
            read_value::<ComponentInstanceReaderV1, ComponentInstance>(&instance_json()).unwrap();
        assert_eq!(instance.id().as_str(), "example");
        assert_eq!(instance.position(), Position::new(-10, 20));
    }

    #[test]
    fn converts_instances_in_order_and_rejects_duplicate_ids() {
        let first: RootObject<ComponentInstanceReaderV1> =
            serde_json::from_value(instance_json()).unwrap();
        let mut second_json = instance_json();
        second_json["id"] = json!("another");
        let second: RootObject<ComponentInstanceReaderV1> =
            serde_json::from_value(second_json).unwrap();
        let instances = convert_component_instances(vec![first.document, second.document]).unwrap();
        assert_eq!(instances[0].id().as_str(), "example");
        assert_eq!(instances[1].id().as_str(), "another");
        assert_eq!(
            convert_component_instances(Vec::new()).unwrap(),
            Vec::<ComponentInstance>::new(),
        );

        let repeated = (0..2)
            .map(|_| {
                serde_json::from_value::<RootObject<ComponentInstanceReaderV1>>(instance_json())
                    .unwrap()
                    .document
            })
            .collect();
        let error = convert_component_instances(repeated).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_duplicate_component_id"
        );
    }

    #[test]
    fn rejects_invalid_ids_and_missing_snapshot_fields() {
        let mut invalid = instance_json();
        invalid["id"] = json!("");
        assert_eq!(
            read_value::<ComponentInstanceReaderV1, ComponentInstance>(&invalid)
                .unwrap_err()
                .message_key(),
            "datastore_key_empty",
        );
        let value = instance_json();
        for field in value.as_object().unwrap().keys() {
            let mut invalid = value.clone();
            invalid.as_object_mut().unwrap().remove(field);
            assert!(
                serde_json::from_value::<RootObject<ComponentInstanceReaderV1>>(invalid).is_err(),
                "{field}"
            );
        }
    }
}
