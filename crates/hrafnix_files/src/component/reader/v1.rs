use crate::background_decoration::reader::v1::BackgroundDecorationReaderV1;
use crate::common::deserialize::{deserialize_object, deserialize_objects};
use crate::component::data::ComponentFile;
use crate::foreground_decoration::reader::v1::ForegroundDecorationReaderV1;
use crate::metadata::reader::v1::MetadataReaderV1;
use serde::Deserialize;

/// The version 1 JSON representation of a component.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ComponentReaderV1 {
    /// Descriptive information about the component.
    #[serde(deserialize_with = "deserialize_object")]
    metadata: MetadataReaderV1,
    /// Decorations drawn in front of component contents.
    #[serde(deserialize_with = "deserialize_objects")]
    foreground_decorations: Vec<ForegroundDecorationReaderV1>,
    /// Decorations drawn behind component contents.
    #[serde(deserialize_with = "deserialize_objects")]
    background_decorations: Vec<BackgroundDecorationReaderV1>,
}

impl From<ComponentReaderV1> for ComponentFile {
    fn from(component: ComponentReaderV1) -> Self {
        Self {
            metadata: component.metadata.into(),
            foreground_decorations: component
                .foreground_decorations
                .into_iter()
                .map(Into::into)
                .collect(),
            background_decorations: component
                .background_decorations
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}
