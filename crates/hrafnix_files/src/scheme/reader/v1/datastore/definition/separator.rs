use hrafnix_datastore::definition::SeparatorDefinition;
use serde::Deserialize;

/// Version-1 separator label.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SeparatorDefinitionReaderV1 {
    /// Item description.
    description: String,
}

impl From<SeparatorDefinitionReaderV1> for SeparatorDefinition {
    fn from(separator: SeparatorDefinitionReaderV1) -> Self {
        SeparatorDefinition::new(separator.description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use serde_json::json;

    #[test]
    fn reads_separator_label() {
        let reader = serde_json::from_value::<RootObject<SeparatorDefinitionReaderV1>>(json!({
            "description": "Output settings"
        }))
        .unwrap();
        assert_eq!(
            SeparatorDefinition::from(reader.document),
            SeparatorDefinition::new("Output settings")
        );
    }

    #[test]
    fn requires_description() {
        let error = serde_json::from_value::<RootObject<SeparatorDefinitionReaderV1>>(json!({}))
            .unwrap_err();
        assert!(error.to_string().contains("missing field `description`"));
    }
}
