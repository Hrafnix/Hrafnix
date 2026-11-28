use crate::common::serialize::serialize_shareable_string;
use crate::scheme::writer::datastore::definition::boolean::BooleanDefinitionWriterV1;
use hrafnix_datastore::frozen::BooleanFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen boolean definition and unevaluated expression.
#[derive(Debug, Serialize)]
pub(crate) struct BooleanWriterV1 {
    /// Boolean metadata, default expression, and state labels.
    definition: BooleanDefinitionWriterV1,
    /// Current unevaluated boolean expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    value: ShareableString,
}

impl From<&BooleanFrozen> for BooleanWriterV1 {
    fn from(boolean: &BooleanFrozen) -> Self {
        Self {
            definition: boolean.definition().into(),
            value: boolean.value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::BooleanDefinition;
    use serde_json::json;

    #[test]
    fn writes_definition_and_current_expression() {
        let boolean = BooleanFrozen::new_with_value(
            BooleanDefinition::new_with_descriptions_and_default("Enabled", "On", "Off", "true"),
            "p_enabled",
        );

        assert_eq!(
            serde_json::to_value(BooleanWriterV1::from(&boolean)).unwrap(),
            json!({
                "definition": {
                    "description": "Enabled", "default": "true",
                    "true_description": "On", "false_description": "Off"
                },
                "value": "p_enabled"
            })
        );
    }
}
