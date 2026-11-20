use crate::common::deserialize::deserialize_objects;
use crate::common::validate::ensure_unique_keys;
use crate::datastore::definition::reader::v1::item::ItemDefinitionReaderV1;
use hrafnix_datastore::definition::{ItemDefinitionType, ParameterObjectDefinition};
use hrafnix_keys::ParameterKey;
use hrafnix_message::message::Message;
use serde::{Deserialize, Deserializer};

/// One version-1 parameter with its key and inline definition.
#[derive(Debug, Deserialize)]
struct ParameterItemReaderV1 {
    /// Plain-string parameter identifier, validated during conversion.
    key: String,
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
        Ok(Self { items })
    }
}

impl TryFrom<ParameterDefinitionReaderV1> for ParameterObjectDefinition {
    type Error = Message;

    fn try_from(parameters: ParameterDefinitionReaderV1) -> Result<Self, Self::Error> {
        ensure_unique_keys(
            parameters.items.iter().map(|item| &item.key),
            "files_duplicate_parameter_key",
        )?;
        let mut builder = Self::builder();
        for item in parameters.items {
            builder.insert(
                ParameterKey::new(item.key.into())?,
                ItemDefinitionType::try_from(item.definition)?,
            );
        }
        Ok(builder.finish())
    }
}
