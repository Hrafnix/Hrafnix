use crate::background_decoration::data::BackgroundDecoration;
use crate::foreground_decoration::data::ForegroundDecoration;
use crate::gui_metadata::data::GuiMetadata;
use crate::metadata::data::MetadataFile;

/// Represents User component data from a Hrafnix component file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentFile {
    /// Descriptive information about the component.
    pub metadata: MetadataFile,
    /// Drawing dimensions and icon associated with the component.
    pub gui_metadata: GuiMetadata,
    /// Text and images drawn in front of the component's contents.
    pub foreground_decorations: Vec<ForegroundDecoration>,
    /// Shapes drawn behind the component's contents.
    pub background_decorations: Vec<BackgroundDecoration>,
}
