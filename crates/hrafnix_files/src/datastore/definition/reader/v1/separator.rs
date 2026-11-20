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
