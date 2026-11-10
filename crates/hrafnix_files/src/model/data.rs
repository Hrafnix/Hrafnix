use crate::background_decoration::data::BackgroundDecoration;
use crate::foreground_decoration::data::ForegroundDecoration;
use crate::metadata::data::MetadataFile;

/// Represents User Model data from a Hrafnix model file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelFile {
    /// Descriptive information about the model.
    pub metadata: MetadataFile,
    /// Text and images drawn in front of the model's contents.
    pub foreground_decorations: Vec<ForegroundDecoration>,
    /// Shapes drawn behind the model's contents.
    pub background_decorations: Vec<BackgroundDecoration>,
}
