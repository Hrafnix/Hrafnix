use crate::common::hash::instance_interface_hash;
use crate::graphics::data::Position;
use crate::metadata_gui::data::GuiMetadata;
pub use crate::source::data::{ComponentSource, PackageReference, PackageVersion};
use hrafnix_datastore::frozen::ParameterObjectFrozen;
use hrafnix_keys::ComponentKey;

/// A saved instance of a file-defined or built-in component.
#[derive(Debug, Clone)]
pub struct ComponentInstance {
    /// Identifier unique within the containing model or component.
    id: ComponentKey,
    /// Unresolved reference to the instance's component definition.
    source: ComponentSource,
    /// Position in the containing drawing.
    position: Position,
    /// Saved dimensions, rotation, icon, and icon-local port locations.
    gui_metadata: GuiMetadata,
    /// Ordered parameter definitions and current unevaluated expressions.
    parameters: ParameterObjectFrozen,
    /// BLAKE3 hash of the saved source, GUI, and parameter interface.
    interface_hash: [u8; 32],
}

impl ComponentInstance {
    /// Creates a new component instance and computes its interface hash.
    #[must_use]
    pub(crate) fn new(
        id: ComponentKey,
        source: ComponentSource,
        position: Position,
        gui_metadata: GuiMetadata,
        parameters: ParameterObjectFrozen,
    ) -> Self {
        let interface_hash =
            instance_interface_hash(&source, &gui_metadata, parameters.definition());
        Self {
            id,
            source,
            position,
            gui_metadata,
            parameters,
            interface_hash,
        }
    }

    /// Returns the identifier unique within the containing model or component.
    #[must_use]
    pub const fn id(&self) -> &ComponentKey {
        &self.id
    }

    /// Returns the unresolved reference to the instance's component definition.
    #[must_use]
    pub const fn source(&self) -> &ComponentSource {
        &self.source
    }

    /// Returns the position in the containing drawing.
    #[must_use]
    pub const fn position(&self) -> Position {
        self.position
    }

    /// Returns the saved dimensions, rotation, icon, and icon-local port locations.
    #[must_use]
    pub const fn gui_metadata(&self) -> &GuiMetadata {
        &self.gui_metadata
    }

    /// Returns the ordered parameter definitions and current unevaluated expressions.
    #[must_use]
    pub const fn parameters(&self) -> &ParameterObjectFrozen {
        &self.parameters
    }

    /// Returns the BLAKE3 hash of the saved source, GUI, and parameter interface.
    #[must_use]
    pub const fn interface_hash(&self) -> [u8; 32] {
        self.interface_hash
    }
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
