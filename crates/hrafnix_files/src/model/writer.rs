use crate::metadata::writer::MetadataWriter;
use crate::model::data::ModelFile;
use serde::Serialize;

/// The current model file envelope.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct ModelWriter {
    /// Version tag identifying the serialized schema.
    file_version: String,
    /// Model contents under the named root entry.
    model: ModelDataWriter,
}

/// The current JSON representation of model contents.
#[derive(Debug, Serialize)]
struct ModelDataWriter {
    /// Descriptive information about the model.
    metadata: MetadataWriter,
}

impl From<ModelFile> for ModelWriter {
    fn from(model: ModelFile) -> Self {
        Self {
            file_version: "1".to_owned(),
            model: ModelDataWriter {
                metadata: MetadataWriter::from(&model.metadata),
            },
        }
    }
}
