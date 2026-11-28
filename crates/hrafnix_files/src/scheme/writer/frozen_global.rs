use crate::common::serialize::serialize_global_key;
use crate::scheme::writer::datastore::frozen::item::ItemWriterV1;
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

#[cfg(test)]
mod tests {
    use super::GlobalFrozenWriter;
    use hrafnix_datastore::definition::{GlobalObjectDefinition, StringDefinition};
    use hrafnix_datastore::frozen::GlobalObjectFrozen;
    use hrafnix_keys::global_key;
    use serde_json::json;

    #[test]
    fn writes_values_in_definition_order_and_explicit_empty_arrays() {
        let definition = GlobalObjectDefinition::builder()
            .with(
                global_key!("g_z"),
                StringDefinition::new_with_default("Last", "z"),
            )
            .with(
                global_key!("g_a"),
                StringDefinition::new_with_default("First", "a"),
            )
            .finish();
        let globals = GlobalObjectFrozen::new(definition);
        assert_eq!(
            serde_json::to_value(GlobalFrozenWriter::from(&globals)).unwrap(),
            json!([
                {
                    "key": "g_z", "type": "string",
                    "definition": {"description": "Last", "default": "z"}, "value": "z"
                },
                {
                    "key": "g_a", "type": "string",
                    "definition": {"description": "First", "default": "a"}, "value": "a"
                }
            ]),
        );
        let empty = GlobalObjectFrozen::new(GlobalObjectDefinition::builder().finish());
        assert_eq!(
            serde_json::to_value(GlobalFrozenWriter::from(&empty)).unwrap(),
            json!([])
        );
    }
}
