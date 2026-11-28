use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::boolean::BooleanDefinitionReaderV1;
use hrafnix_datastore::frozen::BooleanFrozen;
use serde::Deserialize;

/// Version-1 frozen boolean definition and unevaluated expression.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BooleanReaderV1 {
    /// Boolean metadata, default expression, and state labels.
    #[serde(deserialize_with = "deserialize_object")]
    definition: BooleanDefinitionReaderV1,
    /// Current unevaluated boolean expression.
    value: String,
}

impl From<BooleanReaderV1> for BooleanFrozen {
    fn from(boolean: BooleanReaderV1) -> Self {
        BooleanFrozen::new_with_value(boolean.definition.into(), boolean.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::BooleanDefinition;
    use serde_json::{Value, json};

    fn document() -> Value {
        json!({
            "definition": {
                "description": "Enabled",
                "default": "true",
                "true_description": "On",
                "false_description": "Off"
            },
            "value": "p_enabled"
        })
    }

    #[test]
    fn reads_definition_and_unevaluated_value() {
        let reader = serde_json::from_value::<RootObject<BooleanReaderV1>>(document()).unwrap();
        let boolean = BooleanFrozen::from(reader.document);

        assert_eq!(boolean.value(), "p_enabled");
        assert_eq!(
            boolean.definition(),
            &BooleanDefinition::new_with_descriptions_and_default("Enabled", "On", "Off", "true")
        );
    }

    #[test]
    fn rejects_non_string_value() {
        let mut value = document();
        value["value"] = json!(true);
        assert!(serde_json::from_value::<RootObject<BooleanReaderV1>>(value).is_err());
    }
}
