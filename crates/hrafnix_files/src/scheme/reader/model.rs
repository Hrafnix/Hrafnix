use crate::common::deserialize::deserialize_object;
use crate::data::model::ModelFile;
use crate::scheme::reader::v1::model::ModelReaderV1;
use hrafnix_message::message::Message;
use serde::Deserialize;

/// Model data selected by the `file-version` tag.
#[derive(Debug, Deserialize)]
#[serde(tag = "file-version", deny_unknown_fields)]
pub(crate) enum ModelReader {
    /// Version 1 model data.
    #[serde(rename = "1")]
    V1 {
        /// Model contents under the named root entry.
        #[serde(deserialize_with = "deserialize_object")]
        model: ModelReaderV1,
    },
}

impl ModelReader {
    /// Converts a versioned model reader and computes its data hash.
    pub(crate) fn try_into_model_file(self) -> Result<ModelFile, Message> {
        match self {
            Self::V1 { model } => model.try_into_model_file(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ModelReader;
    use crate::common::deserialize::RootObject;
    use serde_json::{Value, json};

    /// A minimal document for testing version dispatch.
    fn document() -> Value {
        json!({
            "file-version": "1",
            "model": {
                "metadata": {
                    "display-name": "", "description": "", "author": "",
                    "minimum-version-major": 0, "minimum-version-minor": 0
                },
                "parameter_definition": [], "variable_definition": [],
                "foreground_decorations": [], "background_decorations": [],
                "global_settings": [], "components": [], "connections": [],
                "translations": {"language": "en", "translations": {}}
            }
        })
    }

    #[test]
    fn selects_version_one() {
        let reader: RootObject<ModelReader> = serde_json::from_value(document()).unwrap();
        assert!(matches!(reader.document, ModelReader::V1 { .. }));
    }

    #[test]
    fn rejects_unsupported_or_non_string_versions_and_wrong_roots() {
        for version in [json!("2"), json!(1), json!(null)] {
            let mut value = document();
            value["file-version"] = version;
            assert!(serde_json::from_value::<RootObject<ModelReader>>(value).is_err());
        }
        let mut wrong_root = document();
        let contents = wrong_root.as_object_mut().unwrap().remove("model").unwrap();
        wrong_root["component"] = contents;
        assert!(serde_json::from_value::<RootObject<ModelReader>>(wrong_root).is_err());
    }
}
