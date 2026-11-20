use crate::common::serialize::serialize_shareable_string;
use hrafnix_datastore::definition::BooleanDefinition;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 boolean expression and labels.
#[derive(Debug, Serialize)]
pub(crate) struct BooleanDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Default expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    default: ShareableString,
    /// True-state label.
    #[serde(serialize_with = "serialize_shareable_string")]
    true_description: ShareableString,
    /// False-state label.
    #[serde(serialize_with = "serialize_shareable_string")]
    false_description: ShareableString,
}

impl From<&BooleanDefinition> for BooleanDefinitionWriterV1 {
    fn from(boolean: &BooleanDefinition) -> Self {
        Self {
            description: boolean.description(),
            default: boolean.default_value(),
            true_description: boolean.true_description_ref().clone(),
            false_description: boolean.false_description_ref().clone(),
        }
    }
}
