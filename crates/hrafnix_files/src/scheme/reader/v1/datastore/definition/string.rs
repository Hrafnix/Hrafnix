use hrafnix_datastore::definition::StringDefinition;
use serde::Deserialize;

/// String expression and description.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StringDefinitionReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
}

impl From<StringDefinitionReaderV1> for StringDefinition {
    fn from(string: StringDefinitionReaderV1) -> Self {
        StringDefinition::new_with_default(string.description, string.default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use serde_json::json;

    #[test]
    fn reads_description_and_default_text() {
        let reader = serde_json::from_value::<RootObject<StringDefinitionReaderV1>>(json!({
            "description": "Label", "default": "héllo\n\"world\""
        }))
        .unwrap();
        assert_eq!(
            StringDefinition::from(reader.document),
            StringDefinition::new_with_default("Label", "héllo\n\"world\"")
        );
    }

    #[test]
    fn requires_default() {
        let error = serde_json::from_value::<RootObject<StringDefinitionReaderV1>>(json!({
            "description": "Label"
        }))
        .unwrap_err();
        assert!(error.to_string().contains("missing field `default`"));
    }
}
