use crate::graphics::data::{Dimension, Position};
use hrafnix_shareable_string::ShareableString;

/// A shape behind a model's or component's contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackgroundDecoration {
    /// Identifier of the decoration.
    pub id: ShareableString,
    /// Position in drawing coordinates.
    pub position: Position,
    /// Dimensions in drawing coordinates.
    pub dimensions: Dimension,
    /// The shape of the decoration.
    pub kind: BackgroundDecorationKind,
}

/// The shapes allowed in a background layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundDecorationKind {
    /// A rectangular outline.
    Frame,
}
