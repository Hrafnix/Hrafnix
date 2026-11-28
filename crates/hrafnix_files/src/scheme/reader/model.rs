use crate::common::deserialize::deserialize_object;
use crate::data::model::ModelFile;
use crate::scheme::reader::v1::model::ModelReaderV1;
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

impl ModelReader {
    /// Converts a versioned model reader and computes its data hash.
    pub(crate) fn try_into_model_file(self) -> Result<ModelFile, Message> {
        match self {
            Self::V1 { model } => model.try_into_model_file(),
        }
    }
}
