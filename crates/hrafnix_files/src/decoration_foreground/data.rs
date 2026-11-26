use crate::graphics::data::{Dimension, FontSize, Position};
use camino::{Utf8Path, Utf8PathBuf};
use hrafnix_shareable_string::{ShareableString, SharedStringStore};

/// Text or an image in front of a model's or component's contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForegroundDecoration {
    /// Identifier of the decoration.
    id: ShareableString,
    /// Position in drawing coordinates.
    position: Position,
    /// Dimensions in drawing coordinates.
    dimensions: Dimension,
    /// The content of the decoration, including its type-specific style.
    kind: ForegroundDecorationKind,
}

impl ForegroundDecoration {
    /// Creates a new foreground decoration with the given identifier, bounds, and content.
    #[must_use]
    pub(crate) const fn new(
        id: ShareableString,
        position: Position,
        dimensions: Dimension,
        kind: ForegroundDecorationKind,
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

    /// Returns the content of the decoration, including its type-specific style.
    #[must_use]
    pub const fn kind(&self) -> &ForegroundDecorationKind {
        &self.kind
    }

    /// Returns a copy whose strings are interned in `store`.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            id: store.launder(&self.id),
            position: self.position,
            dimensions: self.dimensions,
            kind: self.kind.launder(store),
        }
    }
}

/// The content allowed in a foreground layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForegroundDecorationKind {
    /// Text displayed within the decoration's bounds.
    Text(TextDecoration),
    /// An image displayed within the decoration's bounds.
    Image(ImageDecoration),
}

impl ForegroundDecorationKind {
    /// Returns a copy whose strings are interned in `store`.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        match self {
            Self::Text(text) => Self::Text(text.launder(store)),
            Self::Image(image) => Self::Image(image.clone()),
        }
    }
}

/// Text content and styling for a foreground decoration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextDecoration {
    /// The displayed text.
    text: ShareableString,
    /// Font size in hundredths of a drawing unit: `2000` represents `20.00`.
    font_size: FontSize,
}

impl TextDecoration {
    /// Creates new text decoration content with the given text and font size.
    #[must_use]
    pub(crate) const fn new(text: ShareableString, font_size: FontSize) -> Self {
        Self { text, font_size }
    }

    /// Returns the displayed text.
    #[must_use]
    pub const fn text(&self) -> &ShareableString {
        &self.text
    }

    /// Returns the font size in hundredths of a drawing unit.
    #[must_use]
    pub const fn font_size(&self) -> FontSize {
        self.font_size
    }

    /// Returns a copy whose strings are interned in `store`.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            text: store.launder(&self.text),
            font_size: self.font_size,
        }
    }
}

/// Image content and fitting for a foreground decoration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageDecoration {
    /// Image path, stored unchanged rather than opened or resolved during file I/O.
    path: Utf8PathBuf,
    /// How the image fits within the decoration's bounds.
    fit: ImageFit,
}

impl ImageDecoration {
    /// Creates new image decoration content with the given path and fit mode.
    #[must_use]
    pub(crate) const fn new(path: Utf8PathBuf, fit: ImageFit) -> Self {
        Self { path, fit }
    }

    /// Returns the image path.
    #[must_use]
    pub fn path(&self) -> &Utf8Path {
        self.path.as_path()
    }

    /// Returns how the image fits within the decoration's bounds.
    #[must_use]
    pub const fn fit(&self) -> ImageFit {
        self.fit
    }
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
