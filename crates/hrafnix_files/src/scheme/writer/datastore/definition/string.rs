use crate::common::serialize::serialize_shareable_string;
use hrafnix_datastore::definition::StringDefinition;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 string expression and description.
#[derive(Debug, Serialize)]
pub(crate) struct StringDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Default expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    default: ShareableString,
}

impl From<&StringDefinition> for StringDefinitionWriterV1 {
    fn from(string: &StringDefinition) -> Self {
        Self {
            description: string.description(),
            default: string.default_value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn writes_plain_strings_and_preserves_special_characters() {
        let definition = StringDefinition::new_with_default("Label", "héllo\n\"world\"");
        assert_eq!(
            serde_json::to_value(StringDefinitionWriterV1::from(&definition)).unwrap(),
            json!({"description": "Label", "default": "héllo\n\"world\""})
        );
    }
}
