use crate::common::deserialize::{deserialize_objects, deserialize_parameter_key};
use crate::datastore::definition::reader::v1::item::ItemDefinitionReaderV1;
use hrafnix_datastore::definition::{ItemDefinitionType, ParameterObjectDefinition};
use hrafnix_keys::ParameterKey;
use serde::{Deserialize, Deserializer, de::Error};
use std::collections::BTreeSet;

/// One version-1 parameter with its validated key and inline definition.
#[derive(Debug, Deserialize)]
struct ParameterItemReaderV1 {
    /// Plain-string parameter identifier.
    #[serde(deserialize_with = "deserialize_parameter_key")]
    key: ParameterKey,
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
        let mut keys = BTreeSet::new();
        for item in &items {
            if !keys.insert(&item.key) {
                return Err(D::Error::custom(format!(
                    "duplicate parameter key `{}`",
                    item.key
                )));
            }
        }
        Ok(Self { items })
    }
}

impl From<ParameterDefinitionReaderV1> for ParameterObjectDefinition {
    fn from(parameters: ParameterDefinitionReaderV1) -> Self {
        let mut builder = Self::builder();
        for item in parameters.items {
            builder.insert(item.key, ItemDefinitionType::from(item.definition));
        }
        builder.finish()
    }
}
