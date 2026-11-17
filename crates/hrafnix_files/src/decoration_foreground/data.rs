use crate::graphics::data::{Dimension, FontSize, Position};
use camino::Utf8PathBuf;
use hrafnix_shareable_string::ShareableString;

/// Text or an image in front of a model's or component's contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForegroundDecoration {
    /// Identifier of the decoration.
    pub id: ShareableString,
    /// Position in drawing coordinates.
    pub position: Position,
    /// Dimensions in drawing coordinates.
    pub dimensions: Dimension,
    /// The content of the decoration, including its type-specific style.
    pub kind: ForegroundDecorationKind,
}

/// The content allowed in a foreground layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForegroundDecorationKind {
    /// Text displayed within the decoration's bounds.
    Text(TextDecoration),
    /// An image displayed within the decoration's bounds.
    Image(ImageDecoration),
}

/// Text content and styling for a foreground decoration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextDecoration {
    /// The displayed text.
    pub text: ShareableString,
    /// Font size in hundredths of a drawing unit: `2000` represents `20.00`.
    pub font_size: FontSize,
}

/// Image content and fitting for a foreground decoration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageDecoration {
    /// Image path, stored unchanged rather than opened or resolved during file I/O.
    pub path: Utf8PathBuf,
    /// How the image fits within the decoration's bounds.
    pub fit: ImageFit,
}

/// How an image fits within its decoration's bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFit {
    /// Preserve the aspect ratio and show the whole image within the bounds.
    Contain,
    /// Preserve the aspect ratio and cover the bounds, cropping if necessary.
    Cover,
    /// Stretch the image to fill the bounds.
    Fill,
}
