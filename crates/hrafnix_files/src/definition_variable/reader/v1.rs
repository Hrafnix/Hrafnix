use crate::common::deserialize::{deserialize_objects, deserialize_variable_key};
use crate::datastore::definition::reader::v1::item::ItemDefinitionReaderV1;
use hrafnix_datastore::definition::{ItemDefinitionType, VariableObjectDefinition};
use hrafnix_keys::VariableKey;
use serde::{Deserialize, Deserializer, de::Error};
use std::collections::BTreeSet;

/// One version-1 variable with its validated key and inline definition.
#[derive(Debug, Deserialize)]
struct VariableItemReaderV1 {
    /// Plain-string variable identifier.
    #[serde(deserialize_with = "deserialize_variable_key")]
    key: VariableKey,
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
        let mut keys = BTreeSet::new();
        for item in &items {
            if !keys.insert(&item.key) {
                return Err(D::Error::custom(format!(
                    "duplicate variable key `{}`",
                    item.key
                )));
            }
        }
        Ok(Self { items })
    }
}

impl From<VariableDefinitionReaderV1> for VariableObjectDefinition {
    fn from(variables: VariableDefinitionReaderV1) -> Self {
        let mut builder = Self::builder();
        for item in variables.items {
            builder.insert(item.key, ItemDefinitionType::from(item.definition));
        }
        builder.finish()
    }
}
