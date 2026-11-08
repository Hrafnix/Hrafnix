use crate::common::deserialize::deserialize_object;
use crate::model::data::ModelFile;
use crate::model::reader::v1::ModelReaderV1;
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

impl From<ModelReader> for ModelFile {
    fn from(document: ModelReader) -> Self {
        match document {
            ModelReader::V1 { model } => model.into(),
        }
    }
}
