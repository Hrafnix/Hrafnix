use crate::common::deserialize::deserialize_objects;
use crate::common::validate::ensure_unique_keys;
use crate::scheme::reader::v1::datastore::frozen::item::ItemReaderV1;
use hrafnix_datastore::definition::GlobalObjectDefinition;
use hrafnix_datastore::frozen::{GlobalObjectFrozen, ItemFrozen};
use hrafnix_keys::GlobalKey;
use hrafnix_message::message::Message;
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;

/// One version-1 global with its key and inline frozen item.
#[derive(Debug, Deserialize)]
struct GlobalItemReaderV1 {
    /// Plain-string global identifier, validated during conversion.
    key: String,
    /// Tagged frozen item with its embedded definition and value.
    #[serde(flatten)]
    item: ItemReaderV1,
}

/// An ordered array of uniquely named frozen globals.
#[derive(Debug)]
pub(crate) struct GlobalFrozenReaderV1 {
    /// Globals in file order.
    items: Vec<GlobalItemReaderV1>,
}

impl<'de> Deserialize<'de> for GlobalFrozenReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let items = deserialize_objects::<GlobalItemReaderV1, D>(deserializer)?;
        Ok(Self { items })
    }
}

impl TryFrom<GlobalFrozenReaderV1> for GlobalObjectFrozen {
    type Error = Message;

    fn try_from(globals: GlobalFrozenReaderV1) -> Result<Self, Self::Error> {
        ensure_unique_keys(
            globals.items.iter().map(|item| &item.key),
            "files_duplicate_global_key",
        )?;
        let mut definition = GlobalObjectDefinition::builder();
        let mut items = BTreeMap::new();
        for global in globals.items {
            let key = GlobalKey::new(global.key.into())?;
            let item = ItemFrozen::try_from(global.item)?;
            definition.insert(key.clone(), item.definition());
            items.insert(key, item);
        }
        Self::new_with_items(definition.finish(), items)
    }
}

#[cfg(test)]
mod tests {
    use super::GlobalFrozenReaderV1;
    use hrafnix_datastore::definition::GlobalObjectDefinition;
    use hrafnix_datastore::frozen::{GlobalObjectFrozen, ItemFrozen};
    use serde_json::{Value, json};

    /// Saved string values in non-alphabetical key order.
    fn globals() -> Value {
        json!([
            {
                "key": "g_z", "type": "string",
                "definition": {"description": "Last", "default": "base z"}, "value": "saved z"
            },
            {
                "key": "g_a", "type": "string",
                "definition": {"description": "First", "default": "base a"}, "value": "saved a"
            }
        ])
    }

    #[test]
    fn preserves_saved_values_definitions_and_order() {
        let reader = serde_json::from_value::<GlobalFrozenReaderV1>(globals()).unwrap();
        let globals = GlobalObjectFrozen::try_from(reader).unwrap();
        assert_eq!(
            globals
                .ordered_iter()
                .map(|(key, _)| key.as_str())
                .collect::<Vec<_>>(),
            ["g_z", "g_a"],
        );
        let ItemFrozen::String(value) = globals.ordered_iter().next().unwrap().1 else {
            panic!("expected a string value");
        };
        assert_eq!(value.value(), "saved z");
        assert_eq!(value.definition().description(), "Last");
        assert_eq!(value.definition().default_value(), "base z");
        let empty = serde_json::from_value::<GlobalFrozenReaderV1>(json!([])).unwrap();
        assert_eq!(
            GlobalObjectFrozen::try_from(empty).unwrap(),
            GlobalObjectFrozen::new(GlobalObjectDefinition::builder().finish())
        );
    }

    #[test]
    fn rejects_duplicate_keys_and_wrong_key_namespaces() {
        for (key, diagnostic) in [
            ("g_z", "files_duplicate_global_key"),
            ("p_wrong", "datastore_key_invalid_prefix"),
        ] {
            let mut value = globals();
            value[1]["key"] = json!(key);
            let reader = serde_json::from_value::<GlobalFrozenReaderV1>(value).unwrap();
            let error = GlobalObjectFrozen::try_from(reader).unwrap_err();
            assert_eq!(error.translate_data().message_key().as_str(), diagnostic);
        }
        assert!(serde_json::from_value::<GlobalFrozenReaderV1>(json!({})).is_err());
    }
}
