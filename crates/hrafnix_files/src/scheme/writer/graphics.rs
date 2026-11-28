use crate::data::graphics::{Dimension, FontSize, Position, Rotation};
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

/// The current JSON representation of an integer quarter-turn rotation.
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub(crate) struct RotationWriter {
    /// Clockwise rotation in degrees.
    degrees: u16,
}

impl From<&Rotation> for RotationWriter {
    fn from(rotation: &Rotation) -> Self {
        Self {
            degrees: rotation.degrees(),
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

#[cfg(test)]
mod tests {
    use super::{DimensionWriter, FontSizeWriter, PositionWriter, RotationWriter};
    use crate::data::graphics::{Dimension, FontSize, Position, Rotation};
    use serde_json::json;

    #[test]
    fn writes_positions_and_dimensions_as_named_integer_fields() {
        assert_eq!(
            serde_json::to_value(PositionWriter::from(&Position::new(i32::MIN, i32::MAX))).unwrap(),
            json!({"x": i32::MIN, "y": i32::MAX}),
        );
        assert_eq!(
            serde_json::to_value(DimensionWriter::from(&Dimension::new(0, u16::MAX))).unwrap(),
            json!({"width": 0, "height": u16::MAX}),
        );
    }

    #[test]
    fn writes_rotations_and_font_sizes_as_plain_integers() {
        for (rotation, degrees) in [
            (Rotation::Degrees0, 0),
            (Rotation::Degrees90, 90),
            (Rotation::Degrees180, 180),
            (Rotation::Degrees270, 270),
        ] {
            assert_eq!(
                serde_json::to_value(RotationWriter::from(&rotation)).unwrap(),
                json!(degrees)
            );
        }
        for size in [0, 1234, u16::MAX] {
            assert_eq!(
                serde_json::to_value(FontSizeWriter::from(&FontSize::new(size))).unwrap(),
                json!(size),
            );
        }
    }
}
