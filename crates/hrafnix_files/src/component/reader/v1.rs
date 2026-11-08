use crate::common::deserialize::deserialize_object;
use crate::component::data::ComponentFile;
use crate::metadata::reader::v1::MetadataReaderV1;
use serde::Deserialize;

/// The version 1 JSON representation of a component.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ComponentReaderV1 {
    /// Descriptive information about the component.
    #[serde(deserialize_with = "deserialize_object")]
    metadata: MetadataReaderV1,
}

impl From<ComponentReaderV1> for ComponentFile {
    fn from(component: ComponentReaderV1) -> Self {
        Self {
            metadata: component.metadata.into(),
        }
    }
}
