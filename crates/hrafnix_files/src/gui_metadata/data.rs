use crate::graphics::data::Dimension;
use camino::Utf8PathBuf;

/// Drawing dimensions and icon associated with a component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiMetadata {
    /// Dimensions in drawing coordinates.
    pub dimensions: Dimension,
    /// Icon path, stored unchanged rather than opened or resolved during file I/O.
    pub icon: Utf8PathBuf,
}
