use crate::common::deserialize::{deserialize_object, deserialize_objects};
use crate::data::model::ModelFile;
use crate::scheme::reader::v1::component_instance::{
    ComponentInstanceReaderV1, convert_component_instances,
};
use crate::scheme::reader::v1::connection::ConnectionReaderV1;
use crate::scheme::reader::v1::decoration_background::BackgroundDecorationReaderV1;
use crate::scheme::reader::v1::decoration_foreground::ForegroundDecorationReaderV1;
use crate::scheme::reader::v1::definition_parameter::ParameterDefinitionReaderV1;
use crate::scheme::reader::v1::definition_variable::VariableDefinitionReaderV1;
use crate::scheme::reader::v1::frozen_global::GlobalFrozenReaderV1;
use crate::scheme::reader::v1::metadata_general::MetadataReaderV1;
use crate::scheme::reader::v1::translations::TranslationsReaderV1;
use hrafnix_message::message::Message;
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
    /// Required to be ordered parameter-definition array.
    parameter_definition: ParameterDefinitionReaderV1,
    /// Required to be ordered variable-definition array.
    variable_definition: VariableDefinitionReaderV1,
    /// Decorations drawn in front of model contents.
    #[serde(deserialize_with = "deserialize_objects")]
    foreground_decorations: Vec<ForegroundDecorationReaderV1>,
    /// Decorations drawn behind model contents.
    #[serde(deserialize_with = "deserialize_objects")]
    background_decorations: Vec<BackgroundDecorationReaderV1>,
    /// Ordered global settings with embedded definitions and frozen values.
    global_settings: GlobalFrozenReaderV1,
    /// Ordered component instances with unique identifiers.
    #[serde(deserialize_with = "deserialize_objects")]
    components: Vec<ComponentInstanceReaderV1>,
    /// Connections between component ports within the model.
    #[serde(deserialize_with = "deserialize_objects")]
    connections: Vec<ConnectionReaderV1>,
    /// Source language and translated text used by the model.
    #[serde(deserialize_with = "deserialize_object")]
    translations: TranslationsReaderV1,
}

impl ModelReaderV1 {
    /// Converts a version 1 model reader into a model file.
    ///
    /// # Errors
    ///
    /// Returns a localized error if any contained data is invalid.
    pub(crate) fn try_into_model_file(self) -> Result<ModelFile, Message> {
        Ok(ModelFile::new(
            self.metadata.into(),
            self.foreground_decorations
                .into_iter()
                .map(Into::into)
                .collect(),
            self.background_decorations
                .into_iter()
                .map(Into::into)
                .collect(),
            self.global_settings.try_into()?,
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
    use super::ModelReaderV1;
    use crate::common::deserialize::RootObject;
    use serde_json::{Value, json};

    /// Minimal model contents, without populated nested schemas.
    fn contents() -> Value {
        json!({
            "metadata": {
                "display-name": "Example", "description": "", "author": "",
                "minimum-version-major": 0, "minimum-version-minor": 0
            },
            "parameter_definition": [], "variable_definition": [],
            "foreground_decorations": [], "background_decorations": [],
            "global_settings": [], "components": [], "connections": [],
            "translations": {"language": "en", "translations": {}}
        })
    }

    #[test]
    fn converts_minimal_model() {
        let reader: RootObject<ModelReaderV1> = serde_json::from_value(contents()).unwrap();
        let model = reader.document.try_into_model_file().unwrap();
        assert_eq!(model.metadata().display_name(), "Example");
        assert_eq!(model.components(), &[]);
        assert_eq!(model.connections(), &[]);
        assert_eq!(model.global_settings().definition().count(), 0);
    }

    #[test]
    fn requires_all_fields_and_rejects_unknown_fields() {
        let value = contents();
        for field in value.as_object().unwrap().keys() {
            let mut invalid = value.clone();
            invalid.as_object_mut().unwrap().remove(field);
            assert!(
                serde_json::from_value::<RootObject<ModelReaderV1>>(invalid).is_err(),
                "{field}"
            );
        }
        let mut invalid = value;
        invalid["unknown"] = json!(true);
        assert!(serde_json::from_value::<RootObject<ModelReaderV1>>(invalid).is_err());
    }
}
