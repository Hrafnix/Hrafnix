use crate::graphics::data::{Dimension, Position};
use hrafnix_shareable_string::{ShareableString, SharedStringStore};

/// A shape behind a model's or component's contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackgroundDecoration {
    /// Identifier of the decoration.
    id: ShareableString,
    /// Position in drawing coordinates.
    position: Position,
    /// Dimensions in drawing coordinates.
    dimensions: Dimension,
    /// The shape of the decoration.
    kind: BackgroundDecorationKind,
}

impl BackgroundDecoration {
    /// Creates a new background decoration with the given identifier, bounds, and shape.
    #[must_use]
    pub(crate) const fn new(
        id: ShareableString,
        position: Position,
        dimensions: Dimension,
        kind: BackgroundDecorationKind,
    ) -> Self {
        Self {
            id,
            position,
            dimensions,
            kind,
        }
    }

    /// Returns the identifier of the decoration.
    #[must_use]
    pub const fn id(&self) -> &ShareableString {
        &self.id
    }

    /// Returns the position in drawing coordinates.
    #[must_use]
    pub const fn position(&self) -> Position {
        self.position
    }

    /// Returns the dimensions in drawing coordinates.
    #[must_use]
    pub const fn dimensions(&self) -> Dimension {
        self.dimensions
    }

    /// Returns the shape of the decoration.
    #[must_use]
    pub const fn kind(&self) -> &BackgroundDecorationKind {
        &self.kind
    }

    /// Returns a copy whose strings are interned in `store`.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            id: store.launder(&self.id),
            position: self.position,
            dimensions: self.dimensions,
            kind: self.kind.clone(),
        }
    }
}

/// The shapes allowed in a background layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundDecorationKind {
    /// A rectangular outline.
    Frame,
}
