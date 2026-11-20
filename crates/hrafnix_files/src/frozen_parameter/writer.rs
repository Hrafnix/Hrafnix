use crate::common::serialize::serialize_parameter_key;
use crate::datastore::frozen::writer::v1::item::ItemWriterV1;
use hrafnix_datastore::frozen::ParameterObjectFrozen;
use hrafnix_keys::ParameterKey;
use serde::Serialize;

/// One parameter with its plain-string key and inline tagged frozen item.
#[derive(Debug, Serialize)]
struct ParameterItemWriter {
    /// Validated parameter identifier.
    #[serde(serialize_with = "serialize_parameter_key")]
    key: ParameterKey,
    /// Tagged frozen item with its embedded definition and value.
    #[serde(flatten)]
    item: ItemWriterV1,
}

/// Current JSON array representation of frozen parameters.
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub(crate) struct ParameterFrozenWriter {
    /// Parameters in definition order, including an explicit empty array.
    items: Vec<ParameterItemWriter>,
}

impl From<&ParameterObjectFrozen> for ParameterFrozenWriter {
    fn from(parameters: &ParameterObjectFrozen) -> Self {
        Self {
            items: parameters
                .ordered_iter()
                .map(|(key, item)| ParameterItemWriter {
                    key: key.clone(),
                    item: item.into(),
                })
                .collect(),
        }
    }
}
