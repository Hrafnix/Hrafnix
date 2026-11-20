use crate::common::deserialize::{deserialize_object, deserialize_objects};
use crate::component::data::ComponentFile;
use crate::component_instance::reader::v1::{
    ComponentInstanceReaderV1, convert_component_instances,
};
use crate::connection::reader::v1::ConnectionReaderV1;
use crate::decoration_background::reader::v1::BackgroundDecorationReaderV1;
use crate::decoration_foreground::reader::v1::ForegroundDecorationReaderV1;
use crate::definition_parameter::reader::v1::ParameterDefinitionReaderV1;
use crate::definition_variable::reader::v1::VariableDefinitionReaderV1;
use crate::metadata_general::reader::v1::MetadataReaderV1;
use crate::metadata_gui::reader::v1::GuiMetadataReaderV1;
use crate::translations::reader::v1::TranslationsReaderV1;
use hrafnix_message::message::Message;
use serde::Deserialize;

/// The version 1 JSON representation of a component.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ComponentReaderV1 {
    /// Descriptive information about the component.
    #[serde(deserialize_with = "deserialize_object")]
    metadata: MetadataReaderV1,
    /// Drawing dimensions and icon associated with the component.
    #[serde(deserialize_with = "deserialize_object")]
    gui_metadata: GuiMetadataReaderV1,
    /// Required to be ordered parameter-definition array.
    parameter_definition: ParameterDefinitionReaderV1,
    /// Required to be ordered variable-definition array.
    variable_definition: VariableDefinitionReaderV1,
    /// Decorations drawn in front of component contents.
    #[serde(deserialize_with = "deserialize_objects")]
    foreground_decorations: Vec<ForegroundDecorationReaderV1>,
    /// Decorations drawn behind component contents.
    #[serde(deserialize_with = "deserialize_objects")]
    background_decorations: Vec<BackgroundDecorationReaderV1>,
    /// Ordered nested component instances with unique identifiers.
    #[serde(deserialize_with = "deserialize_objects")]
    components: Vec<ComponentInstanceReaderV1>,
    /// Connections between component ports within the component.
    #[serde(deserialize_with = "deserialize_objects")]
    connections: Vec<ConnectionReaderV1>,
    /// Source language and translated text used by the component.
    #[serde(deserialize_with = "deserialize_object")]
    translations: TranslationsReaderV1,
}

impl TryFrom<ComponentReaderV1> for ComponentFile {
    type Error = Message;

    fn try_from(component: ComponentReaderV1) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: component.metadata.into(),
            gui_metadata: component.gui_metadata.try_into()?,
            parameter_definition: component.parameter_definition.try_into()?,
            variable_definition: component.variable_definition.try_into()?,
            foreground_decorations: component
                .foreground_decorations
                .into_iter()
                .map(Into::into)
                .collect(),
            background_decorations: component
                .background_decorations
                .into_iter()
                .map(Into::into)
                .collect(),
            translations: component.translations.try_into()?,
            components: convert_component_instances(component.components)?,
            connections: component
                .connections
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}
