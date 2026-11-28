use crate::common::deserialize::deserialize_objects;
use crate::common::validate::ensure_unique_keys;
use crate::scheme::reader::v1::datastore::definition::item::ItemDefinitionReaderV1;
use hrafnix_datastore::definition::{ItemDefinitionType, VariableObjectDefinition};
use hrafnix_keys::VariableKey;
use hrafnix_message::message::Message;
use serde::{Deserialize, Deserializer};

/// One version-1 variable with its key and inline definition.
#[derive(Debug, Deserialize)]
struct VariableItemReaderV1 {
    /// Plain-string variable identifier, validated during conversion.
    key: String,
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
        Ok(Self { items })
    }
}

impl TryFrom<VariableDefinitionReaderV1> for VariableObjectDefinition {
    type Error = Message;

    fn try_from(variables: VariableDefinitionReaderV1) -> Result<Self, Self::Error> {
        ensure_unique_keys(
            variables.items.iter().map(|item| &item.key),
            "files_duplicate_variable_key",
        )?;
        let mut builder = Self::builder();
        for item in variables.items {
            builder.insert(
                VariableKey::new(item.key.into())?,
                ItemDefinitionType::try_from(item.definition)?,
            );
        }
        Ok(builder.finish())
    }
}
