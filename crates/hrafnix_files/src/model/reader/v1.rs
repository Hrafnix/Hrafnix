use crate::background_decoration::reader::v1::BackgroundDecorationReaderV1;
use crate::common::deserialize::{deserialize_object, deserialize_objects};
use crate::connection::reader::v1::ConnectionReaderV1;
use crate::foreground_decoration::reader::v1::ForegroundDecorationReaderV1;
use crate::metadata::reader::v1::MetadataReaderV1;
use crate::model::data::ModelFile;
use crate::translations::reader::v1::TranslationsReaderV1;
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
    #[serde(deserialize_with = "deserialize_object")]
    metadata: MetadataReaderV1,
    /// Decorations drawn in front of model contents.
    #[serde(deserialize_with = "deserialize_objects")]
    foreground_decorations: Vec<ForegroundDecorationReaderV1>,
    /// Decorations drawn behind model contents.
    #[serde(deserialize_with = "deserialize_objects")]
    background_decorations: Vec<BackgroundDecorationReaderV1>,
    /// Connections between component ports within the model.
    #[serde(deserialize_with = "deserialize_objects")]
    connections: Vec<ConnectionReaderV1>,
    /// Source language and translated text used by the model.
    #[serde(deserialize_with = "deserialize_object")]
    translations: TranslationsReaderV1,
}

impl From<ModelReaderV1> for ModelFile {
    fn from(model: ModelReaderV1) -> Self {
        Self {
            metadata: model.metadata.into(),
            foreground_decorations: model
                .foreground_decorations
                .into_iter()
                .map(Into::into)
                .collect(),
            background_decorations: model
                .background_decorations
                .into_iter()
                .map(Into::into)
                .collect(),
            translations: model.translations.into(),
            connections: model.connections.into_iter().map(Into::into).collect(),
        }
    }
}
