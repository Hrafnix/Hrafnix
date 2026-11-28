use crate::common::deserialize::deserialize_object;
use crate::data::component::ComponentFile;
use crate::data::source::ComponentSource;
use crate::scheme::reader::v1::component::ComponentReaderV1;
use hrafnix_message::message::Message;
use serde::Deserialize;

/// Component data selected by the `file-version` tag.
#[derive(Debug, Deserialize)]
#[serde(tag = "file-version", deny_unknown_fields)]
pub(crate) enum ComponentReader {
    /// Version 1 component data.
    #[serde(rename = "1")]
    V1 {
        /// Component contents under the named root entry.
        #[serde(deserialize_with = "deserialize_object")]
        component: ComponentReaderV1,
    },
}

impl ComponentReader {
    /// Converts a versioned component reader and computes its data hashes.
    pub(crate) fn try_into_component_file(
        self,
        source: ComponentSource,
    ) -> Result<ComponentFile, Message> {
        match self {
            Self::V1 { component } => component.try_into_component_file(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentReader;
    use crate::common::deserialize::RootObject;
    use serde_json::{Value, json};

    /// A minimal document for testing version dispatch.
    fn document() -> Value {
        json!({
            "file-version": "1",
            "component": {
                "metadata": {
                    "display-name": "", "description": "", "author": "",
                    "minimum-version-major": 0, "minimum-version-minor": 0
                },
                "gui_metadata": {
                    "dimensions": {"width": 0, "height": 0},
                    "rotation": 0, "icon": "", "ports": []
                },
                "parameter_definition": [], "variable_definition": [],
                "foreground_decorations": [], "background_decorations": [],
                "components": [], "connections": [],
                "translations": {"language": "en", "translations": {}}
            }
        })
    }

    #[test]
    fn selects_version_one() {
        let reader: RootObject<ComponentReader> = serde_json::from_value(document()).unwrap();
        assert!(matches!(reader.document, ComponentReader::V1 { .. }));
    }

    #[test]
    fn rejects_unsupported_or_non_string_versions_and_wrong_roots() {
        for version in [json!("2"), json!(1), json!(null)] {
            let mut value = document();
            value["file-version"] = version;
            assert!(serde_json::from_value::<RootObject<ComponentReader>>(value).is_err());
        }
        let mut wrong_root = document();
        let contents = wrong_root
            .as_object_mut()
            .unwrap()
            .remove("component")
            .unwrap();
        wrong_root["model"] = contents;
        assert!(serde_json::from_value::<RootObject<ComponentReader>>(wrong_root).is_err());
    }
}
