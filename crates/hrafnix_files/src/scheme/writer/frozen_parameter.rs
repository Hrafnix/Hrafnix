use crate::common::serialize::serialize_parameter_key;
use crate::scheme::writer::datastore::frozen::item::ItemWriterV1;
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

#[cfg(test)]
mod tests {
    use super::ParameterFrozenWriter;
    use hrafnix_datastore::definition::{ParameterObjectDefinition, StringDefinition};
    use hrafnix_datastore::frozen::ParameterObjectFrozen;
    use hrafnix_keys::parameter_key;
    use serde_json::json;

    #[test]
    fn writes_values_in_definition_order_and_explicit_empty_arrays() {
        let definition = ParameterObjectDefinition::builder()
            .with(
                parameter_key!("p_z"),
                StringDefinition::new_with_default("Last", "z"),
            )
            .with(
                parameter_key!("p_a"),
                StringDefinition::new_with_default("First", "a"),
            )
            .finish();
        let parameters = ParameterObjectFrozen::new(definition);
        assert_eq!(
            serde_json::to_value(ParameterFrozenWriter::from(&parameters)).unwrap(),
            json!([
                {
                    "key": "p_z", "type": "string",
                    "definition": {"description": "Last", "default": "z"}, "value": "z"
                },
                {
                    "key": "p_a", "type": "string",
                    "definition": {"description": "First", "default": "a"}, "value": "a"
                }
            ]),
        );
        let empty = ParameterObjectFrozen::new(ParameterObjectDefinition::builder().finish());
        assert_eq!(
            serde_json::to_value(ParameterFrozenWriter::from(&empty)).unwrap(),
            json!([])
        );
    }
}
