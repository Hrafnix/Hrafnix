use crate::common::deserialize::deserialize_objects;
use crate::common::validate::ensure_unique_keys;
use crate::datastore::frozen::reader::v1::item::ItemReaderV1;
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
