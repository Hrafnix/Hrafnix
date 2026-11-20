use crate::common::deserialize::deserialize_object;
use crate::model::data::ModelFile;
use crate::model::reader::v1::ModelReaderV1;
use hrafnix_message::message::Message;
use serde::Deserialize;

/// Model data selected by the `file-version` tag.
#[derive(Debug, Deserialize)]
#[serde(tag = "file-version", deny_unknown_fields)]
pub(crate) enum ModelReader {
    /// Version 1 model data.
    #[serde(rename = "1")]
    V1 {
        /// Model contents under the named root entry.
        #[serde(deserialize_with = "deserialize_object")]
        model: ModelReaderV1,
    },
}

impl TryFrom<ModelReader> for ModelFile {
    type Error = Message;

    fn try_from(document: ModelReader) -> Result<Self, Self::Error> {
        match document {
            ModelReader::V1 { model } => model.try_into(),
        }
    }
}
