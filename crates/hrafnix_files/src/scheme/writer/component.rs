use crate::data::component::ComponentFile;
use crate::scheme::writer::component_instance::ComponentInstanceWriter;
use crate::scheme::writer::connection::ConnectionWriter;
use crate::scheme::writer::decoration_background::BackgroundDecorationWriter;
use crate::scheme::writer::decoration_foreground::ForegroundDecorationWriter;
use crate::scheme::writer::definition_parameter::ParameterDefinitionWriter;
use crate::scheme::writer::definition_variable::VariableDefinitionWriter;
use crate::scheme::writer::metadata_general::MetadataWriter;
use crate::scheme::writer::metadata_gui::GuiMetadataWriter;
use crate::scheme::writer::translations::TranslationsWriter;
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
    /// Drawing dimensions, rotation, icon, and port locations associated with the component.
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
                metadata: MetadataWriter::from(component.metadata()),
                gui_metadata: GuiMetadataWriter::from(component.gui_metadata()),
                parameter_definition: ParameterDefinitionWriter::from(
                    component.parameter_definition(),
                ),
                variable_definition: VariableDefinitionWriter::from(
                    component.variable_definition(),
                ),
                foreground_decorations: component
                    .foreground_decorations()
                    .iter()
                    .map(Into::into)
                    .collect(),
                background_decorations: component
                    .background_decorations()
                    .iter()
                    .map(Into::into)
                    .collect(),
                translations: TranslationsWriter::from(component.translations()),
                components: component.components().iter().map(Into::into).collect(),
                connections: component.connections().iter().map(Into::into).collect(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentWriter;
    use crate::data::component::ComponentFile;
    use crate::data::graphics::{Dimension, Rotation};
    use crate::data::metadata_general::MetadataFile;
    use crate::data::metadata_gui::GuiMetadata;
    use crate::data::source::ComponentSource;
    use crate::data::translations::TranslationsFile;
    use hrafnix_datastore::definition::{ParameterObjectDefinition, VariableObjectDefinition};
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn writes_version_envelope_and_required_empty_collections() {
        let component = ComponentFile::new(
            MetadataFile::new(
                "Example".into(),
                "Description".into(),
                "Author".into(),
                1,
                2,
            ),
            ComponentSource::File {
                package: None,
                path: "example.hxc".into(),
            },
            GuiMetadata::new(
                Dimension::new(10, 20),
                Rotation::Degrees90,
                "icon.svg".into(),
                Vec::new(),
            ),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            TranslationsFile::new("en".into(), BTreeMap::new()),
            ParameterObjectDefinition::builder().finish(),
            VariableObjectDefinition::builder().finish(),
        );
        assert_eq!(
            serde_json::to_value(ComponentWriter::from(component)).unwrap(),
            json!({
                "file-version": "1",
                "component": {
                    "metadata": {
                        "display-name": "Example", "description": "Description", "author": "Author",
                        "minimum-version-major": 1, "minimum-version-minor": 2
                    },
                    "gui_metadata": {
                        "dimensions": {"width": 10, "height": 20},
                        "rotation": 90, "icon": "icon.svg", "ports": []
                    },
                    "parameter_definition": [], "variable_definition": [],
                    "foreground_decorations": [], "background_decorations": [],
                    "components": [], "connections": [],
                    "translations": {"language": "en", "translations": {}}
                }
            }),
        );
    }
}
