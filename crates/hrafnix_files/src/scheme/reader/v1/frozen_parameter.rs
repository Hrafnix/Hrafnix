use crate::common::deserialize::deserialize_objects;
use crate::common::validate::ensure_unique_keys;
use crate::scheme::reader::v1::datastore::frozen::item::ItemReaderV1;
use hrafnix_datastore::definition::ParameterObjectDefinition;
use hrafnix_datastore::frozen::{ItemFrozen, ParameterObjectFrozen};
use hrafnix_keys::ParameterKey;
use hrafnix_message::message::Message;
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;

/// One version-1 parameter with its key and inline frozen item.
#[derive(Debug, Deserialize)]
struct ParameterItemReaderV1 {
    /// Plain-string parameter identifier, validated during conversion.
    key: String,
    /// Tagged frozen item with its embedded definition and value.
    #[serde(flatten)]
    item: ItemReaderV1,
}

/// An ordered array of uniquely named frozen parameters.
#[derive(Debug)]
pub(crate) struct ParameterFrozenReaderV1 {
    /// Parameters in file order.
    items: Vec<ParameterItemReaderV1>,
}

impl<'de> Deserialize<'de> for ParameterFrozenReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let items = deserialize_objects::<ParameterItemReaderV1, D>(deserializer)?;
        Ok(Self { items })
    }
}

impl TryFrom<ParameterFrozenReaderV1> for ParameterObjectFrozen {
    type Error = Message;

    fn try_from(parameters: ParameterFrozenReaderV1) -> Result<Self, Self::Error> {
        ensure_unique_keys(
            parameters.items.iter().map(|item| &item.key),
            "files_duplicate_parameter_key",
        )?;
        let mut definition = ParameterObjectDefinition::builder();
        let mut items = BTreeMap::new();
        for parameter in parameters.items {
            let key = ParameterKey::new(parameter.key.into())?;
            let item = ItemFrozen::try_from(parameter.item)?;
            definition.insert(key.clone(), item.definition());
            items.insert(key, item);
        }
        Self::new_with_items(definition.finish(), items)
    }
}

#[cfg(test)]
mod tests {
    use super::ParameterFrozenReaderV1;
    use hrafnix_datastore::definition::ParameterObjectDefinition;
    use hrafnix_datastore::frozen::{ItemFrozen, ParameterObjectFrozen};
    use serde_json::{Value, json};

    /// Saved string values in non-alphabetical key order.
    fn parameters() -> Value {
        json!([
            {
                "key": "p_z", "type": "string",
                "definition": {"description": "Last", "default": "base z"}, "value": "saved z"
            },
            {
                "key": "p_a", "type": "string",
                "definition": {"description": "First", "default": "base a"}, "value": "saved a"
            }
        ])
    }

    #[test]
    fn preserves_saved_values_definitions_and_order() {
        let reader = serde_json::from_value::<ParameterFrozenReaderV1>(parameters()).unwrap();
        let parameters = ParameterObjectFrozen::try_from(reader).unwrap();
        assert_eq!(
            parameters
                .ordered_iter()
                .map(|(key, _)| key.as_str())
                .collect::<Vec<_>>(),
            ["p_z", "p_a"],
        );
        let ItemFrozen::String(value) = parameters.ordered_iter().next().unwrap().1 else {
            panic!("expected a string value");
        };
        assert_eq!(value.value(), "saved z");
        assert_eq!(value.definition().description(), "Last");
        assert_eq!(value.definition().default_value(), "base z");
        let empty = serde_json::from_value::<ParameterFrozenReaderV1>(json!([])).unwrap();
        assert_eq!(
            ParameterObjectFrozen::try_from(empty).unwrap(),
            ParameterObjectFrozen::new(ParameterObjectDefinition::builder().finish())
        );
    }

    #[test]
    fn rejects_duplicate_keys_and_wrong_key_namespaces() {
        for (key, diagnostic) in [
            ("p_z", "files_duplicate_parameter_key"),
            ("g_wrong", "datastore_key_invalid_prefix"),
        ] {
            let mut value = parameters();
            value[1]["key"] = json!(key);
            let reader = serde_json::from_value::<ParameterFrozenReaderV1>(value).unwrap();
            let error = ParameterObjectFrozen::try_from(reader).unwrap_err();
            assert_eq!(error.translate_data().message_key().as_str(), diagnostic);
        }
        assert!(serde_json::from_value::<ParameterFrozenReaderV1>(json!({})).is_err());
    }
}
