use crate::graphics::data::{Color, CornerRadius, Dimension, FontSize, Position, StrokeWidth};
use serde::{Serialize, Serializer};

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

/// The current JSON representation of a stroke width in hundredths.
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub(crate) struct StrokeWidthWriter {
    /// Width in hundredths.
    width: u16,
}

impl From<&StrokeWidth> for StrokeWidthWriter {
    fn from(stroke_width: &StrokeWidth) -> Self {
        Self {
            width: stroke_width.width(),
        }
    }
}

/// The current JSON representation of a corner radius in hundredths.
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub(crate) struct CornerRadiusWriter {
    /// Radius in hundredths.
    radius: u16,
}

impl From<&CornerRadius> for CornerRadiusWriter {
    fn from(corner_radius: &CornerRadius) -> Self {
        Self {
            radius: corner_radius.radius(),
        }
    }
}

/// The current theme-default or hex-string JSON representation of a color.
#[derive(Debug)]
pub(crate) enum ColorWriter {
    /// The theme's color for the role.
    ThemeDefault,
    /// An explicit color with alpha last.
    Custom {
        /// Red channel.
        red: u8,
        /// Green channel.
        green: u8,
        /// Blue channel.
        blue: u8,
        /// Alpha channel.
        alpha: u8,
    },
}

impl From<&Color> for ColorWriter {
    fn from(color: &Color) -> Self {
        match color {
            Color::ThemeDefault => Self::ThemeDefault,
            Color::Custom(rgba) => Self::Custom {
                red: rgba.red(),
                green: rgba.green(),
                blue: rgba.blue(),
                alpha: rgba.alpha(),
            },
        }
    }
}

impl Serialize for ColorWriter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            Self::ThemeDefault => serializer.serialize_str("theme-default"),
            Self::Custom {
                red,
                green,
                blue,
                alpha,
            } => {
                if alpha == u8::MAX {
                    serializer.collect_str(&format_args!("#{red:02x}{green:02x}{blue:02x}"))
                } else {
                    serializer
                        .collect_str(&format_args!("#{red:02x}{green:02x}{blue:02x}{alpha:02x}"))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ColorWriter;
    use crate::graphics::data::{Color, Rgba};

    #[test]
    fn writes_theme_default() {
        assert_eq!(
            serde_json::to_string(&ColorWriter::from(&Color::ThemeDefault)).unwrap(),
            r#""theme-default""#
        );
    }

    #[test]
    fn writes_canonical_hex_colors() {
        for (color, expected) in [
            (Rgba::new(0, 0, 0, 255), r##""#000000""##),
            (Rgba::new(255, 255, 255, 255), r##""#ffffff""##),
            (Rgba::new(170, 187, 204, 255), r##""#aabbcc""##),
            (Rgba::new(1, 2, 3, 4), r##""#01020304""##),
            (Rgba::new(170, 187, 204, 0), r##""#aabbcc00""##),
            (Rgba::new(170, 187, 204, 254), r##""#aabbccfe""##),
            (Rgba::new(0, 0, 0, 0), r##""#00000000""##),
        ] {
            let writer = ColorWriter::from(&Color::Custom(color));
            assert_eq!(serde_json::to_string(&writer).unwrap(), expected);
        }
    }
}
