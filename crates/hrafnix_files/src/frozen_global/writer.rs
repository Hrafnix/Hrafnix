use crate::common::serialize::serialize_global_key;
use crate::datastore::frozen::writer::v1::item::ItemWriterV1;
use hrafnix_datastore::frozen::GlobalObjectFrozen;
use hrafnix_keys::GlobalKey;
use serde::Serialize;

/// One global with its plain-string key and inline tagged frozen item.
#[derive(Debug, Serialize)]
struct GlobalItemWriter {
    /// Validated global identifier.
    #[serde(serialize_with = "serialize_global_key")]
    key: GlobalKey,
    /// Tagged frozen item with its embedded definition and value.
    #[serde(flatten)]
    item: ItemWriterV1,
}

/// Current JSON array representation of frozen globals.
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub(crate) struct GlobalFrozenWriter {
    /// Globals in definition order, including an explicit empty array.
    items: Vec<GlobalItemWriter>,
}

impl From<&GlobalObjectFrozen> for GlobalFrozenWriter {
    fn from(globals: &GlobalObjectFrozen) -> Self {
        Self {
            items: globals
                .ordered_iter()
                .map(|(key, item)| GlobalItemWriter {
                    key: key.clone(),
                    item: item.into(),
                })
                .collect(),
        }
    }
}
