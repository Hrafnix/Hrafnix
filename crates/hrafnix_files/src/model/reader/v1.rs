use crate::metadata::reader::v1::MetadataReaderV1;
use crate::model::data::ModelFile;
use serde::Deserialize;

/// The version 1 JSON representation of a model, separate from the
/// application's model data.
///
/// Unknown fields are rejected so unexpected content is an error instead of
/// being silently dropped.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ModelReaderV1 {
    /// Descriptive information about the model.
    metadata: MetadataReaderV1,
}

impl From<ModelReaderV1> for ModelFile {
    fn from(model: ModelReaderV1) -> Self {
        Self {
            metadata: model.metadata.into(),
        }
    }
}
