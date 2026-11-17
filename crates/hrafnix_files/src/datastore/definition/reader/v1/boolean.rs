use hrafnix_datastore::definition::BooleanDefinition;
use serde::Deserialize;

/// Boolean expression and labels.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BooleanReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// True-state label.
    true_description: String,
    /// False-state label.
    false_description: String,
}

impl From<BooleanReaderV1> for BooleanDefinition {
    fn from(boolean: BooleanReaderV1) -> Self {
        BooleanDefinition::new_with_descriptions_and_default(
            boolean.description,
            boolean.true_description,
            boolean.false_description,
            boolean.default,
        )
    }
}
