use hrafnix_datastore::definition::StringDefinition;
use serde::Deserialize;

/// String expression and description.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StringReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
}

impl From<StringReaderV1> for StringDefinition {
    fn from(string: StringReaderV1) -> Self {
        StringDefinition::new_with_default(string.description, string.default)
    }
}
