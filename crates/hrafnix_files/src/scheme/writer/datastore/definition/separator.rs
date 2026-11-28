use crate::common::serialize::serialize_shareable_string;
use hrafnix_datastore::definition::SeparatorDefinition;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 separator label.
#[derive(Debug, Serialize)]
pub(crate) struct SeparatorDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
}

impl From<&SeparatorDefinition> for SeparatorDefinitionWriterV1 {
    fn from(separator: &SeparatorDefinition) -> Self {
        Self {
            description: separator.description(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn writes_only_the_separator_label() {
        let definition = SeparatorDefinition::new("Output settings");
        assert_eq!(
            serde_json::to_value(SeparatorDefinitionWriterV1::from(&definition)).unwrap(),
            json!({"description": "Output settings"})
        );
    }
}
