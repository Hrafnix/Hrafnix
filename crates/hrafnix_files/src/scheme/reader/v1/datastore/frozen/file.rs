use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::file::FileDefinitionReaderV1;
use hrafnix_datastore::frozen::FileFrozen;
use serde::Deserialize;

/// Version-1 frozen file definition and unevaluated path expression.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FileReaderV1 {
    /// File metadata, default expression, and dialog options.
    #[serde(deserialize_with = "deserialize_object")]
    definition: FileDefinitionReaderV1,
    /// Current unevaluated file path expression.
    value: String,
}

impl From<FileReaderV1> for FileFrozen {
    fn from(file: FileReaderV1) -> Self {
        FileFrozen::new_with_value(file.definition.into(), file.value)
    }
}
