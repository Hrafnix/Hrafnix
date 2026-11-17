use crate::common::serialize::serialize_variable_key;
use crate::datastore::definition::writer::item::ItemDefinitionWriter;
use hrafnix_datastore::definition::VariableObjectDefinition;
use hrafnix_keys::VariableKey;
use serde::Serialize;

/// One variable with its plain-string key and inline tagged definition.
#[derive(Debug, Serialize)]
struct VariableItemWriter {
    /// Validated variable identifier.
    #[serde(serialize_with = "serialize_variable_key")]
    key: VariableKey,
    /// Tagged definition.
    #[serde(flatten)]
    definition: ItemDefinitionWriter,
}

/// Current JSON array representation of variable definitions.
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub(crate) struct VariableDefinitionWriter {
    /// Variables in datastore iteration order, including an explicit empty array.
    items: Vec<VariableItemWriter>,
}

impl From<&VariableObjectDefinition> for VariableDefinitionWriter {
    fn from(variables: &VariableObjectDefinition) -> Self {
        Self {
            items: variables
                .iter()
                .map(|(key, definition)| VariableItemWriter {
                    key: key.clone(),
                    definition: definition.into(),
                })
                .collect(),
        }
    }
}
