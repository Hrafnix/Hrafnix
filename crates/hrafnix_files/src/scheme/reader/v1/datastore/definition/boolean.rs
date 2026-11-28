use hrafnix_datastore::definition::BooleanDefinition;
use serde::Deserialize;

/// Boolean expression and labels.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BooleanDefinitionReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// True-state label.
    true_description: String,
    /// False-state label.
    false_description: String,
}

impl From<BooleanDefinitionReaderV1> for BooleanDefinition {
    fn from(boolean: BooleanDefinitionReaderV1) -> Self {
        BooleanDefinition::new_with_descriptions_and_default(
            boolean.description,
            boolean.true_description,
            boolean.false_description,
            boolean.default,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use serde_json::json;

    #[test]
    fn reads_expression_and_state_labels() {
        let reader = serde_json::from_value::<RootObject<BooleanDefinitionReaderV1>>(json!({
            "description": "Enabled",
            "default": "1 == 1",
            "true_description": "On",
            "false_description": "Off"
        }))
        .unwrap();
        assert_eq!(
            BooleanDefinition::from(reader.document),
            BooleanDefinition::new_with_descriptions_and_default("Enabled", "On", "Off", "1 == 1")
        );
    }

    #[test]
    fn rejects_non_string_default() {
        assert!(
            serde_json::from_value::<RootObject<BooleanDefinitionReaderV1>>(json!({
                "description": "Enabled", "default": true,
                "true_description": "On", "false_description": "Off"
            }))
            .unwrap_err()
            .to_string()
            .contains("expected a string")
        );
    }
}
