use crate::component::data::ComponentFile;
use crate::component_instance::writer::{ComponentInstanceWriter, serialize_component_instances};
use crate::connection::writer::ConnectionWriter;
use crate::decoration_background::writer::BackgroundDecorationWriter;
use crate::decoration_foreground::writer::ForegroundDecorationWriter;
use crate::definition_parameter::writer::ParameterDefinitionWriter;
use crate::definition_variable::writer::VariableDefinitionWriter;
use crate::metadata_general::writer::MetadataWriter;
use crate::metadata_gui::writer::GuiMetadataWriter;
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
    /// Parameter definitions in datastore iteration order.
    parameter_definition: ParameterDefinitionWriter,
    /// Variable definitions in datastore iteration order.
    variable_definition: VariableDefinitionWriter,
    /// Decorations drawn in front of component contents.
    foreground_decorations: Vec<ForegroundDecorationWriter>,
    /// Decorations drawn behind component contents.
    background_decorations: Vec<BackgroundDecorationWriter>,
    /// Ordered saved nested instances with unique identifiers.
    #[serde(serialize_with = "serialize_component_instances")]
    components: Vec<ComponentInstanceWriter>,
    /// Connections between component ports within the component.
    connections: Vec<ConnectionWriter>,
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
                parameter_definition: ParameterDefinitionWriter::from(
                    &component.parameter_definition,
                ),
                variable_definition: VariableDefinitionWriter::from(&component.variable_definition),
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
                components: component.components.iter().map(Into::into).collect(),
                connections: component.connections.iter().map(Into::into).collect(),
            },
        }
    }
}
