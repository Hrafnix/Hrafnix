use crate::common::serialize::serialize_parameter_key;
use crate::scheme::writer::datastore::definition::item::ItemDefinitionWriter;
use hrafnix_datastore::definition::ParameterObjectDefinition;
use hrafnix_keys::ParameterKey;
use serde::Serialize;

/// One parameter with its plain-string key and inline tagged definition.
#[derive(Debug, Serialize)]
struct ParameterItemWriter {
    /// Validated parameter identifier.
    #[serde(serialize_with = "serialize_parameter_key")]
    key: ParameterKey,
    /// Tagged definition.
    #[serde(flatten)]
    definition: ItemDefinitionWriter,
}

/// Current JSON array representation of parameter definitions.
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub(crate) struct ParameterDefinitionWriter {
    /// Parameters in datastore iteration order, including an explicit empty array.
    items: Vec<ParameterItemWriter>,
}

impl From<&ParameterObjectDefinition> for ParameterDefinitionWriter {
    fn from(parameters: &ParameterObjectDefinition) -> Self {
        Self {
            items: parameters
                .iter()
                .map(|(key, definition)| ParameterItemWriter {
                    key: key.clone(),
                    definition: definition.into(),
                })
                .collect(),
        }
    }
}
