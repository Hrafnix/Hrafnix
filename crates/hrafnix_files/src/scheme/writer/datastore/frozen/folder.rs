use crate::common::serialize::serialize_shareable_string;
use crate::scheme::writer::datastore::definition::folder::FolderDefinitionWriterV1;
use hrafnix_datastore::frozen::FolderFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen folder definition and unevaluated path expression.
#[derive(Debug, Serialize)]
pub(crate) struct FolderWriterV1 {
    /// Folder metadata, default expression, and archive option.
    definition: FolderDefinitionWriterV1,
    /// Current unevaluated folder path expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    value: ShareableString,
}

impl From<&FolderFrozen> for FolderWriterV1 {
    fn from(folder: &FolderFrozen) -> Self {
        Self {
            definition: folder.definition().into(),
            value: folder.value(),
        }
    }
}
