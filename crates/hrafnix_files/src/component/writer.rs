use crate::component::data::ComponentFile;
use crate::metadata::writer::MetadataWriter;
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
}

impl From<ComponentFile> for ComponentWriter {
    fn from(component: ComponentFile) -> Self {
        Self {
            file_version: "1".to_owned(),
            component: ComponentDataWriter {
                metadata: MetadataWriter::from(&component.metadata),
            },
        }
    }
}
