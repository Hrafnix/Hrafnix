use crate::common::deserialize::deserialize_object;
use crate::component::data::ComponentFile;
use crate::component::reader::v1::ComponentReaderV1;
use hrafnix_message::message::Message;
use serde::Deserialize;

/// Component data selected by the `file-version` tag.
#[derive(Debug, Deserialize)]
#[serde(tag = "file-version", deny_unknown_fields)]
pub(crate) enum ComponentReader {
    /// Version 1 component data.
    #[serde(rename = "1")]
    V1 {
        /// Component contents under the named root entry.
        #[serde(deserialize_with = "deserialize_object")]
        component: ComponentReaderV1,
    },
}

impl TryFrom<ComponentReader> for ComponentFile {
    type Error = Message;

    fn try_from(document: ComponentReader) -> Result<Self, Self::Error> {
        match document {
            ComponentReader::V1 { component } => component.try_into(),
        }
    }
}
