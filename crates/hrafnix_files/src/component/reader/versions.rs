use crate::component::data::ComponentFile;
use crate::component::reader::v1::ComponentReaderV1;
use serde::Deserialize;

/// Component data selected by the `file-version` tag.
#[derive(Debug, Deserialize)]
#[serde(tag = "file-version", deny_unknown_fields)]
pub(crate) enum ComponentReader {
    /// Version 1 component data.
    #[serde(rename = "1")]
    V1 {
        /// Component contents under the named root entry.
        component: ComponentReaderV1,
    },
}

impl From<ComponentReader> for ComponentFile {
    fn from(document: ComponentReader) -> Self {
        match document {
            ComponentReader::V1 { component } => component.into(),
        }
    }
}
