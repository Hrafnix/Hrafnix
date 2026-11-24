use crate::graphics::data::Position;
use crate::metadata_gui::data::GuiMetadata;
pub use crate::source::data::{ComponentSource, PackageReference};
use hrafnix_datastore::frozen::ParameterObjectFrozen;
use hrafnix_keys::ComponentKey;

/// A saved instance of a file-defined or built-in component.
#[derive(Debug, Clone)]
pub struct ComponentInstance {
    /// Identifier unique within the containing model or component.
    pub id: ComponentKey,
    /// Unresolved reference to the instance's component definition.
    pub source: ComponentSource,
    /// Position in the containing drawing.
    pub position: Position,
    /// Saved dimensions, icon, and icon-local port locations.
    pub gui_metadata: GuiMetadata,
    /// Ordered parameter definitions and current unevaluated expressions.
    pub parameters: ParameterObjectFrozen,
    /// BLAKE3 hash of the saved source, GUI, and parameter interface.
    pub interface_hash: [u8; 32],
}

impl PartialEq for ComponentInstance {
    fn eq(&self, other: &Self) -> bool {
        self.interface_hash == other.interface_hash
            && self.id == other.id
            && self.source == other.source
            && self.position == other.position
            && self.gui_metadata == other.gui_metadata
            && self.parameters == other.parameters
    }
}
