use crate::common::deserialize::deserialize_objects;
use crate::common::validate::ensure_unique_keys;
use crate::scheme::reader::v1::datastore::definition::item::ItemDefinitionReaderV1;
use hrafnix_datastore::definition::{ItemDefinitionType, ParameterObjectDefinition};
use hrafnix_keys::ParameterKey;
use hrafnix_message::message::Message;
use serde::{Deserialize, Deserializer};

/// One version-1 parameter with its key and inline definition.
#[derive(Debug, Deserialize)]
struct ParameterItemReaderV1 {
    /// Plain-string parameter identifier, validated during conversion.
    key: String,
    /// Tagged definition; each variant rejects unknown fields.
    #[serde(flatten)]
    definition: ItemDefinitionReaderV1,
}

/// A required ordered array of uniquely named parameter definitions.
#[derive(Debug)]
pub(crate) struct ParameterDefinitionReaderV1 {
    /// Parameters in file order.
    items: Vec<ParameterItemReaderV1>,
}

impl<'de> Deserialize<'de> for ParameterDefinitionReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let items = deserialize_objects::<ParameterItemReaderV1, D>(deserializer)?;
        Ok(Self { items })
    }
}

impl TryFrom<ParameterDefinitionReaderV1> for ParameterObjectDefinition {
    type Error = Message;

    fn try_from(parameters: ParameterDefinitionReaderV1) -> Result<Self, Self::Error> {
        ensure_unique_keys(
            parameters.items.iter().map(|item| &item.key),
            "files_duplicate_parameter_key",
        )?;
        let mut builder = Self::builder();
        for item in parameters.items {
            builder.insert(
                ParameterKey::new(item.key.into())?,
                ItemDefinitionType::try_from(item.definition)?,
            );
        }
        Ok(builder.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::ParameterDefinitionReaderV1;
    use hrafnix_datastore::definition::{ItemDefinitionType, ParameterObjectDefinition};
    use serde_json::{Value, json};

    /// Two definitions in non-alphabetical key order.
    fn definitions() -> Value {
        json!([
            {"key": "p_z", "type": "string", "description": "Last", "default": "base z"},
            {"key": "p_a", "type": "string", "description": "First", "default": "base a"}
        ])
    }

    #[test]
    fn preserves_definition_fields_and_insertion_order() {
        let reader = serde_json::from_value::<ParameterDefinitionReaderV1>(definitions()).unwrap();
        let parameters = ParameterObjectDefinition::try_from(reader).unwrap();
        assert_eq!(
            parameters
                .iter()
                .map(|(key, _)| key.as_str())
                .collect::<Vec<_>>(),
            ["p_z", "p_a"],
        );
        let ItemDefinitionType::String(definition) = parameters.iter().next().unwrap().1 else {
            panic!("expected a string definition");
        };
        assert_eq!(definition.description(), "Last");
        assert_eq!(definition.default_value(), "base z");
        let empty = serde_json::from_value::<ParameterDefinitionReaderV1>(json!([])).unwrap();
        assert_eq!(
            ParameterObjectDefinition::try_from(empty).unwrap().count(),
            0
        );
    }

    #[test]
    fn rejects_duplicate_keys_and_wrong_key_namespaces() {
        for (key, diagnostic) in [
            ("p_z", "files_duplicate_parameter_key"),
            ("v_wrong", "datastore_key_invalid_prefix"),
        ] {
            let mut value = definitions();
            value[1]["key"] = json!(key);
            let reader = serde_json::from_value::<ParameterDefinitionReaderV1>(value).unwrap();
            let error = ParameterObjectDefinition::try_from(reader).unwrap_err();
            assert_eq!(error.translate_data().message_key().as_str(), diagnostic);
        }
        assert!(serde_json::from_value::<ParameterDefinitionReaderV1>(json!({})).is_err());
    }
}
