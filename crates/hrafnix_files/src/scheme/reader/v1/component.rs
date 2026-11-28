use crate::common::deserialize::{deserialize_object, deserialize_objects};
use crate::data::component::ComponentFile;
use crate::data::source::ComponentSource;
use crate::scheme::reader::v1::component_instance::{
    ComponentInstanceReaderV1, convert_component_instances,
};
use crate::scheme::reader::v1::connection::ConnectionReaderV1;
use crate::scheme::reader::v1::decoration_background::BackgroundDecorationReaderV1;
use crate::scheme::reader::v1::decoration_foreground::ForegroundDecorationReaderV1;
use crate::scheme::reader::v1::definition_parameter::ParameterDefinitionReaderV1;
use crate::scheme::reader::v1::definition_variable::VariableDefinitionReaderV1;
use crate::scheme::reader::v1::metadata_general::MetadataReaderV1;
use crate::scheme::reader::v1::metadata_gui::GuiMetadataReaderV1;
use crate::scheme::reader::v1::translations::TranslationsReaderV1;
use hrafnix_message::message::Message;
use serde::Deserialize;

/// The version 1 JSON representation of a component.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ComponentReaderV1 {
    /// Descriptive information about the component.
    #[serde(deserialize_with = "deserialize_object")]
    metadata: MetadataReaderV1,
    /// Drawing dimensions, rotation, icon, and port locations associated with the component.
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

impl ComponentReaderV1 {
    /// Converts a version 1 component reader into a component file.
    ///
    /// # Errors
    ///
    /// Returns a localized error if any of the contained data is invalid.
    pub(crate) fn try_into_component_file(
        self,
        source: ComponentSource,
    ) -> Result<ComponentFile, Message> {
        Ok(ComponentFile::new(
            self.metadata.into(),
            source,
            self.gui_metadata.try_into()?,
            self.foreground_decorations
                .into_iter()
                .map(Into::into)
                .collect(),
            self.background_decorations
                .into_iter()
                .map(Into::into)
                .collect(),
            convert_component_instances(self.components)?,
            self.connections
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            self.translations.try_into()?,
            self.parameter_definition.try_into()?,
            self.variable_definition.try_into()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentReaderV1;
    use crate::common::deserialize::RootObject;
    use crate::data::source::ComponentSource;
    use serde_json::{Value, json};

    /// Minimal component contents, without populated nested schemas.
    fn contents() -> Value {
        json!({
            "metadata": {
                "display-name": "Example", "description": "", "author": "",
                "minimum-version-major": 0, "minimum-version-minor": 0
            },
            "gui_metadata": {
                "dimensions": {"width": 10, "height": 20},
                "rotation": 0, "icon": "", "ports": []
            },
            "parameter_definition": [], "variable_definition": [],
            "foreground_decorations": [], "background_decorations": [],
            "components": [], "connections": [],
            "translations": {"language": "en", "translations": {}}
        })
    }

    #[test]
    fn converts_minimal_component_and_attaches_supplied_source() {
        let reader: RootObject<ComponentReaderV1> = serde_json::from_value(contents()).unwrap();
        let source = ComponentSource::File {
            package: None,
            path: "example.hxc".into(),
        };
        let component = reader
            .document
            .try_into_component_file(source.clone())
            .unwrap();
        assert_eq!(component.source(), &source);
        assert_eq!(component.metadata().display_name(), "Example");
        assert_eq!(component.components(), &[]);
        assert_eq!(component.connections(), &[]);
    }

    #[test]
    fn requires_all_fields_and_rejects_unknown_fields() {
        let value = contents();
        for field in value.as_object().unwrap().keys() {
            let mut invalid = value.clone();
            invalid.as_object_mut().unwrap().remove(field);
            assert!(
                serde_json::from_value::<RootObject<ComponentReaderV1>>(invalid).is_err(),
                "{field}"
            );
        }
        let mut invalid = value;
        invalid["unknown"] = json!(true);
        assert!(serde_json::from_value::<RootObject<ComponentReaderV1>>(invalid).is_err());
    }
}
