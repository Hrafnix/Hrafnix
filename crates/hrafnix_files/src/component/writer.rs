use crate::background_decoration::writer::BackgroundDecorationWriter;
use crate::component::data::ComponentFile;
use crate::foreground_decoration::writer::ForegroundDecorationWriter;
use crate::gui_metadata::writer::GuiMetadataWriter;
use crate::metadata::writer::MetadataWriter;
use crate::translations::writer::TranslationsWriter;
use serde::Serialize;

/// The current component file envelope.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct ComponentWriter {
    /// Version tag identifying the serialized schema.
    file_version: String,
    /// Component contents under the named root entry.
    component: ComponentDataWriter,
}

/// The current JSON representation of component contents.
#[derive(Debug, Serialize)]
struct ComponentDataWriter {
    /// Descriptive information about the component.
    metadata: MetadataWriter,
    /// Drawing dimensions and icon associated with the component.
    gui_metadata: GuiMetadataWriter,
    /// Decorations drawn in front of component contents.
    foreground_decorations: Vec<ForegroundDecorationWriter>,
    /// Decorations drawn behind component contents.
    background_decorations: Vec<BackgroundDecorationWriter>,
    /// Source language and translated text used by the component.
    translations: TranslationsWriter,
}

impl From<ComponentFile> for ComponentWriter {
    fn from(component: ComponentFile) -> Self {
        Self {
            file_version: "1".to_owned(),
            component: ComponentDataWriter {
                metadata: MetadataWriter::from(&component.metadata),
                gui_metadata: GuiMetadataWriter::from(&component.gui_metadata),
                foreground_decorations: component
                    .foreground_decorations
                    .iter()
                    .map(Into::into)
                    .collect(),
                background_decorations: component
                    .background_decorations
                    .iter()
                    .map(Into::into)
                    .collect(),
                translations: TranslationsWriter::from(&component.translations),
            },
        }
    }
}
