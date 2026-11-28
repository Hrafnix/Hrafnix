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
