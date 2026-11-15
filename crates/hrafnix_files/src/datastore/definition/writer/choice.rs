use crate::common::serialize::{serialize_shareable_string, serialize_store_key};
use hrafnix_datastore::definition::{ChoiceDefinition, ChoiceItemDefinition};
use hrafnix_keys::StoreKey;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 choice identifier and label.
#[derive(Debug, Serialize)]
pub(crate) struct ChoiceItemWriterV1 {
    /// Plain-string choice identifier.
    #[serde(serialize_with = "serialize_store_key")]
    id: StoreKey,
    /// Choice label.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
}

impl From<&ChoiceItemDefinition> for ChoiceItemWriterV1 {
    fn from(item: &ChoiceItemDefinition) -> Self {
        Self {
            id: item.id(),
            description: item.description(),
        }
    }
}

/// Version-1 choice expression and ordered choices.
#[derive(Debug, Serialize)]
pub(crate) struct ChoiceWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Default expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    default: ShareableString,
    /// Choice items.
    items: Vec<ChoiceItemWriterV1>,
}

impl From<&ChoiceDefinition> for ChoiceWriterV1 {
    fn from(choice: &ChoiceDefinition) -> Self {
        Self {
            description: choice.description(),
            default: choice.default_value(),
            items: choice.choices().iter().map(Into::into).collect(),
        }
    }
}
