use crate::common::deserialize::{deserialize_objects, deserialize_store_key};
use hrafnix_datastore::definition::{ChoiceDefinition, ChoiceItemDefinition};
use hrafnix_keys::StoreKey;
use serde::Deserialize;

/// Version-1 choice item.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ChoiceItemReaderV1 {
    /// Validated choice identifier.
    #[serde(deserialize_with = "deserialize_store_key")]
    id: StoreKey,
    /// Choice label.
    description: String,
}

/// Version-1 choice definition.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ChoiceReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// Ordered choices; the datastore definition normalizes duplicate IDs.
    #[serde(deserialize_with = "deserialize_objects")]
    items: Vec<ChoiceItemReaderV1>,
}

impl From<ChoiceReaderV1> for ChoiceDefinition {
    fn from(choice: ChoiceReaderV1) -> Self {
        let items = choice
            .items
            .into_iter()
            .map(|item| ChoiceItemDefinition::new(item.id, item.description))
            .collect();
        ChoiceDefinition::new_with_default(choice.description, items, choice.default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use serde_json::json;

    #[test]
    fn normalizes_duplicate_choice_ids_through_the_definition() {
        let value = json!({
            "description": "Mode",
            "default": "fast",
            "items": [
                {"id": "fast", "description": "Fast"},
                {"id": "slow", "description": "Slow"},
                {"id": "fast", "description": "Also fast"}
            ]
        });
        let reader = serde_json::from_value::<RootObject<ChoiceReaderV1>>(value).unwrap();
        let choice = ChoiceDefinition::from(reader.document);
        assert_eq!(choice.ids(), vec!["slow", "fast"]);
        assert_eq!(choice.descriptions(), vec!["Slow", "Also fast"]);
        assert_eq!(choice.default_value(), "fast");
    }
}
