use crate::scheme::writer::datastore::definition::separator::SeparatorDefinitionWriterV1;
use hrafnix_datastore::frozen::SeparatorFrozen;
use serde::Serialize;

/// Version-1 frozen separator, which has metadata but no value.
#[derive(Debug, Serialize)]
pub(crate) struct SeparatorWriterV1 {
    /// Separator label.
    definition: SeparatorDefinitionWriterV1,
}

impl From<&SeparatorFrozen> for SeparatorWriterV1 {
    fn from(separator: &SeparatorFrozen) -> Self {
        Self {
            definition: separator.definition().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::SeparatorDefinition;
    use serde_json::json;

    #[test]
    fn writes_only_definition_metadata() {
        let separator = SeparatorFrozen::new(SeparatorDefinition::new("Advanced"));

        assert_eq!(
            serde_json::to_value(SeparatorWriterV1::from(&separator)).unwrap(),
            json!({"definition": {"description": "Advanced"}})
        );
    }
}
