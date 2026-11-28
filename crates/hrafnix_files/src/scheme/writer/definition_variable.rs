use crate::common::serialize::serialize_variable_key;
use crate::scheme::writer::datastore::definition::item::ItemDefinitionWriter;
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

#[cfg(test)]
mod tests {
    use super::VariableDefinitionWriter;
    use hrafnix_datastore::definition::{StringDefinition, VariableObjectDefinition};
    use hrafnix_keys::variable_key;
    use serde_json::json;

    #[test]
    fn writes_flat_definitions_in_insertion_order() {
        let variables = VariableObjectDefinition::builder()
            .with(
                variable_key!("v_z"),
                StringDefinition::new_with_default("Last", "base z"),
            )
            .with(
                variable_key!("v_a"),
                StringDefinition::new_with_default("First", "base a"),
            )
            .finish();
        assert_eq!(
            serde_json::to_value(VariableDefinitionWriter::from(&variables)).unwrap(),
            json!([
                {"key": "v_z", "type": "string", "description": "Last", "default": "base z"},
                {"key": "v_a", "type": "string", "description": "First", "default": "base a"}
            ]),
        );
        assert_eq!(
            serde_json::to_value(VariableDefinitionWriter::from(
                &VariableObjectDefinition::builder().finish()
            ))
            .unwrap(),
            json!([]),
        );
    }
}
