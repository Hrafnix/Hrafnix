use crate::common::deserialize::{deserialize_global_key, deserialize_objects};
use crate::datastore::frozen::reader::v1::item::ItemReaderV1;
use hrafnix_datastore::definition::GlobalObjectDefinition;
use hrafnix_datastore::frozen::{GlobalObjectFrozen, ItemFrozen};
use hrafnix_keys::GlobalKey;
use hrafnix_message::message::Message;
use serde::{Deserialize, Deserializer, de::Error};
use std::collections::{BTreeMap, BTreeSet};

/// One version-1 global with its validated key and inline frozen item.
#[derive(Debug, Deserialize)]
struct GlobalItemReaderV1 {
    /// Plain-string global identifier.
    #[serde(deserialize_with = "deserialize_global_key")]
    key: GlobalKey,
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
        let mut keys = BTreeSet::new();
        for item in &items {
            if !keys.insert(&item.key) {
                return Err(D::Error::custom(format!(
                    "duplicate global key `{}`",
                    item.key
                )));
            }
        }
        Ok(Self { items })
    }
}

impl TryFrom<GlobalFrozenReaderV1> for GlobalObjectFrozen {
    type Error = Message;

    fn try_from(globals: GlobalFrozenReaderV1) -> Result<Self, Self::Error> {
        let mut definition = GlobalObjectDefinition::builder();
        let mut items = BTreeMap::new();
        for global in globals.items {
            let item = ItemFrozen::try_from(global.item)?;
            definition.insert(global.key.clone(), item.definition());
            items.insert(global.key, item);
        }
        Self::new_with_items(definition.finish(), items)
    }
}
