use crate::common::deserialize::deserialize_object;
use crate::component::data::ComponentFile;
use crate::scheme::reader::v1::component::ComponentReaderV1;
use crate::source::data::ComponentSource;
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
