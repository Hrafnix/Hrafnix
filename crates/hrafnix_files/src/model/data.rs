use crate::metadata::data::MetadataFile;

/// Represents User Model data from a Hrafnix model file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelFile {
    /// Descriptive information about the model.
    pub metadata: MetadataFile,
}
