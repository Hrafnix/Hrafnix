use crate::graphics::data::{Dimension, FontSize, Position};
use serde::Serialize;

/// The current JSON representation of a position.
#[derive(Debug, Serialize)]
pub(crate) struct PositionWriter {
    /// Horizontal position.
    x: i32,
    /// Vertical position.
    y: i32,
}

impl From<&Position> for PositionWriter {
    fn from(position: &Position) -> Self {
        Self {
            x: position.x(),
            y: position.y(),
        }
    }
}

/// The current JSON representation of dimensions.
#[derive(Debug, Serialize)]
pub(crate) struct DimensionWriter {
    /// Width.
    width: u16,
    /// Height.
    height: u16,
}

impl From<&Dimension> for DimensionWriter {
    fn from(dimensions: &Dimension) -> Self {
        Self {
            width: dimensions.width(),
            height: dimensions.height(),
        }
    }
}

/// The current JSON representation of a font size in hundredths.
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub(crate) struct FontSizeWriter {
    /// Size in hundredths.
    size: u16,
}

impl From<&FontSize> for FontSizeWriter {
    fn from(font_size: &FontSize) -> Self {
        Self {
            size: font_size.size(),
        }
    }
}
