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
