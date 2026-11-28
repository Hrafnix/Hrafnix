use crate::common::deserialize::deserialize_objects;
use crate::common::validate::ensure_unique_keys;
use crate::scheme::reader::v1::datastore::definition::item::ItemDefinitionReaderV1;
use hrafnix_datastore::definition::{ItemDefinitionType, VariableObjectDefinition};
use hrafnix_keys::VariableKey;
use hrafnix_message::message::Message;
use serde::{Deserialize, Deserializer};

/// One version-1 variable with its key and inline definition.
#[derive(Debug, Deserialize)]
struct VariableItemReaderV1 {
    /// Plain-string variable identifier, validated during conversion.
    key: String,
    /// Tagged definition; each variant rejects unknown fields.
    #[serde(flatten)]
    definition: ItemDefinitionReaderV1,
}

/// A required ordered array of uniquely named variable definitions.
#[derive(Debug)]
pub(crate) struct VariableDefinitionReaderV1 {
    /// Variables in file order.
    items: Vec<VariableItemReaderV1>,
}

impl<'de> Deserialize<'de> for VariableDefinitionReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let items = deserialize_objects::<VariableItemReaderV1, D>(deserializer)?;
        Ok(Self { items })
    }
}

impl TryFrom<VariableDefinitionReaderV1> for VariableObjectDefinition {
    type Error = Message;

    fn try_from(variables: VariableDefinitionReaderV1) -> Result<Self, Self::Error> {
        ensure_unique_keys(
            variables.items.iter().map(|item| &item.key),
            "files_duplicate_variable_key",
        )?;
        let mut builder = Self::builder();
        for item in variables.items {
            builder.insert(
                VariableKey::new(item.key.into())?,
                ItemDefinitionType::try_from(item.definition)?,
            );
        }
        Ok(builder.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::VariableDefinitionReaderV1;
    use hrafnix_datastore::definition::{ItemDefinitionType, VariableObjectDefinition};
    use serde_json::{Value, json};

    /// Two definitions in non-alphabetical key order.
    fn definitions() -> Value {
        json!([
            {"key": "v_z", "type": "string", "description": "Last", "default": "base z"},
            {"key": "v_a", "type": "string", "description": "First", "default": "base a"}
        ])
    }

    #[test]
    fn preserves_definition_fields_and_insertion_order() {
        let reader = serde_json::from_value::<VariableDefinitionReaderV1>(definitions()).unwrap();
        let variables = VariableObjectDefinition::try_from(reader).unwrap();
        assert_eq!(
            variables
                .iter()
                .map(|(key, _)| key.as_str())
                .collect::<Vec<_>>(),
            ["v_z", "v_a"],
        );
        let ItemDefinitionType::String(definition) = variables.iter().next().unwrap().1 else {
            panic!("expected a string definition");
        };
        assert_eq!(definition.description(), "Last");
        assert_eq!(definition.default_value(), "base z");
        let empty = serde_json::from_value::<VariableDefinitionReaderV1>(json!([])).unwrap();
        assert_eq!(
            VariableObjectDefinition::try_from(empty).unwrap().count(),
            0
        );
    }

    #[test]
    fn rejects_duplicate_keys_and_wrong_key_namespaces() {
        for (key, diagnostic) in [
            ("v_z", "files_duplicate_variable_key"),
            ("p_wrong", "datastore_key_invalid_prefix"),
        ] {
            let mut value = definitions();
            value[1]["key"] = json!(key);
            let reader = serde_json::from_value::<VariableDefinitionReaderV1>(value).unwrap();
            let error = VariableObjectDefinition::try_from(reader).unwrap_err();
            assert_eq!(error.translate_data().message_key().as_str(), diagnostic);
        }
        assert!(serde_json::from_value::<VariableDefinitionReaderV1>(json!({})).is_err());
    }
}
