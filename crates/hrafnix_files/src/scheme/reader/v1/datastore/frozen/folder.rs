use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::folder::FolderDefinitionReaderV1;
use hrafnix_datastore::frozen::FolderFrozen;
use serde::Deserialize;

/// Version-1 frozen folder definition and unevaluated path expression.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FolderReaderV1 {
    /// Folder metadata, default expression, and archive option.
    #[serde(deserialize_with = "deserialize_object")]
    definition: FolderDefinitionReaderV1,
    /// Current unevaluated folder path expression.
    value: String,
}

impl From<FolderReaderV1> for FolderFrozen {
    fn from(folder: FolderReaderV1) -> Self {
        FolderFrozen::new_with_value(folder.definition.into(), folder.value)
    }
}
