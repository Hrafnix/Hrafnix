use crate::background_decoration::writer::BackgroundDecorationWriter;
use crate::connection::writer::ConnectionWriter;
use crate::foreground_decoration::writer::ForegroundDecorationWriter;
use crate::metadata::writer::MetadataWriter;
use crate::model::data::ModelFile;
use crate::translations::writer::TranslationsWriter;
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
    /// Decorations drawn in front of model contents.
    foreground_decorations: Vec<ForegroundDecorationWriter>,
    /// Decorations drawn behind model contents.
    background_decorations: Vec<BackgroundDecorationWriter>,
    /// Connections between component ports within the model.
    connections: Vec<ConnectionWriter>,
    /// Source language and translated text used by the model.
    translations: TranslationsWriter,
}

impl From<ModelFile> for ModelWriter {
    fn from(model: ModelFile) -> Self {
        Self {
            file_version: "1".to_owned(),
            model: ModelDataWriter {
                metadata: MetadataWriter::from(&model.metadata),
                foreground_decorations: model
                    .foreground_decorations
                    .iter()
                    .map(Into::into)
                    .collect(),
                background_decorations: model
                    .background_decorations
                    .iter()
                    .map(Into::into)
                    .collect(),
                translations: TranslationsWriter::from(&model.translations),
                connections: model.connections.iter().map(Into::into).collect(),
            },
        }
    }
}
