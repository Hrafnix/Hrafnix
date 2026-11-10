use crate::graphics::data::{
    Color, CornerRadius, Dimension, FontSize, Position, Rgba, StrokeWidth,
};
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

/// The version 1 JSON representation of a drawing position.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PositionReaderV1 {
    /// Horizontal position.
    x: i32,
    /// Vertical position.
    y: i32,
}

impl From<PositionReaderV1> for Position {
    fn from(component: PositionReaderV1) -> Self {
        Self::new(component.x, component.y)
    }
}

/// The version 1 JSON representation of drawing dimensions.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DimensionReaderV1 {
    /// Width in drawing coordinates.
    width: u16,
    /// Height in drawing coordinates.
    height: u16,
}

impl From<DimensionReaderV1> for Dimension {
    fn from(component: DimensionReaderV1) -> Self {
        Self::new(component.width, component.height)
    }
}

/// The version 1 JSON representation of a font size in hundredths.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub(crate) struct FontSizeReaderV1 {
    /// Size in hundredths.
    size: u16,
}

impl From<FontSizeReaderV1> for FontSize {
    fn from(component: FontSizeReaderV1) -> Self {
        Self::new(component.size)
    }
}

/// The version 1 JSON representation of a stroke width in hundredths.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub(crate) struct StrokeWidthReaderV1 {
    /// Width in hundredths.
    width: u16,
}

impl From<StrokeWidthReaderV1> for StrokeWidth {
    fn from(component: StrokeWidthReaderV1) -> Self {
        Self::new(component.width)
    }
}

/// The version 1 JSON representation of a corner radius in hundredths.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub(crate) struct CornerRadiusReaderV1 {
    /// Radius in hundredths.
    radius: u16,
}

impl From<CornerRadiusReaderV1> for CornerRadius {
    fn from(component: CornerRadiusReaderV1) -> Self {
        Self::new(component.radius)
    }
}

/// A version 1 color encoded as `"theme-default"`, `#RRGGBB`, or `#RRGGBBAA`.
#[derive(Debug)]
pub(crate) enum ColorReaderV1 {
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

impl<'de> Deserialize<'de> for ColorReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        /// Validates a color while preserving the deserializer's error position.
        struct ColorVisitor;

        impl Visitor<'_> for ColorVisitor {
            type Value = ColorReaderV1;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("\"theme-default\" or a color in #RRGGBB or #RRGGBBAA format")
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
                Self::Value::try_from(value).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(ColorVisitor)
    }
}

impl TryFrom<&str> for ColorReaderV1 {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "theme-default" {
            return Ok(Self::ThemeDefault);
        }

        let invalid_color = "expected \"theme-default\" or a color in #RRGGBB or #RRGGBBAA format";
        let hex = value.strip_prefix('#').ok_or(invalid_color)?;
        if !matches!(hex.len(), 6 | 8) || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(invalid_color);
        }

        let channels = u32::from_str_radix(hex, 16)
            .map_err(|_| invalid_color)?
            .to_be_bytes();
        let [red, green, blue, alpha] = if hex.len() == 6 {
            let [_, red, green, blue] = channels;
            [red, green, blue, u8::MAX]
        } else {
            channels
        };
        Ok(Self::Custom {
            red,
            green,
            blue,
            alpha,
        })
    }
}

impl From<ColorReaderV1> for Color {
    fn from(color: ColorReaderV1) -> Self {
        match color {
            ColorReaderV1::ThemeDefault => Self::ThemeDefault,
            ColorReaderV1::Custom {
                red,
                green,
                blue,
                alpha,
            } => Self::Custom(Rgba::new(red, green, blue, alpha)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ColorReaderV1, CornerRadiusReaderV1, FontSizeReaderV1, StrokeWidthReaderV1};
    use crate::graphics::data::{Color, CornerRadius, FontSize, Rgba, StrokeWidth};

    #[test]
    fn hundredths_are_plain_integers() {
        let font_size: FontSizeReaderV1 = serde_json::from_str("1234").unwrap();
        assert_eq!(FontSize::from(font_size).size(), 1234);
        let stroke_width: StrokeWidthReaderV1 = serde_json::from_str("0").unwrap();
        assert_eq!(StrokeWidth::from(stroke_width).width(), 0);
        let corner_radius: CornerRadiusReaderV1 = serde_json::from_str("65535").unwrap();
        assert_eq!(CornerRadius::from(corner_radius).radius(), u16::MAX);

        for json in [
            "12.34",
            "1e3",
            "1234.0",
            "-1",
            "65536",
            "\"1234\"",
            "null",
            "[1234]",
            r#"{"size": 1234}"#,
        ] {
            assert!(
                serde_json::from_str::<FontSizeReaderV1>(json).is_err(),
                "{json}"
            );
            assert!(
                serde_json::from_str::<StrokeWidthReaderV1>(json).is_err(),
                "{json}"
            );
            assert!(
                serde_json::from_str::<CornerRadiusReaderV1>(json).is_err(),
                "{json}"
            );
        }
    }

    #[test]
    fn reads_theme_default() {
        let reader: ColorReaderV1 = serde_json::from_str(r#""theme-default""#).unwrap();
        assert_eq!(Color::from(reader), Color::ThemeDefault);
    }

    #[test]
    fn reads_rgb_and_rgba_hex_colors() {
        for (json, expected) in [
            (r##""#000000""##, Rgba::new(0, 0, 0, 255)),
            (r##""#ffffff""##, Rgba::new(255, 255, 255, 255)),
            (r##""#AaBbCc""##, Rgba::new(170, 187, 204, 255)),
            (r##""#01020304""##, Rgba::new(1, 2, 3, 4)),
            (r##""#AABBCC00""##, Rgba::new(170, 187, 204, 0)),
            (r##""#aabbccFE""##, Rgba::new(170, 187, 204, 254)),
            (r##""#AABBCCFF""##, Rgba::new(170, 187, 204, 255)),
        ] {
            let reader: ColorReaderV1 = serde_json::from_str(json).unwrap();
            assert_eq!(Color::from(reader), Color::Custom(expected));
        }
    }

    #[test]
    fn rejects_invalid_colors() {
        for value in [
            "",
            "default",
            "ThemeDefault",
            "THEME-DEFAULT",
            "theme_default",
            " theme-default",
            "theme-default ",
            "ffffff",
            "#abc",
            "#abcd",
            "#12345",
            "#1234567",
            "#123456789",
            "#gg0000",
            "#000000gg",
            "#+12345",
            "#+1234567",
            "#000000 ",
            " #000000",
            "#\u{e9}0000",
            "#\u{ff11}\u{ff12}\u{ff13}",
        ] {
            let json = serde_json::to_string(value).unwrap();
            let error = serde_json::from_str::<ColorReaderV1>(&json).unwrap_err();
            assert!(error.is_data(), "{value}");
            assert!(
                error.to_string().contains("#RRGGBB or #RRGGBBAA"),
                "{value}: {error}"
            );
            assert!(error.line() > 0);
            assert!(error.column() > 0);
        }
    }

    #[test]
    fn rejects_non_string_colors() {
        for json in ["null", "123", "true", "{}", "[]", r##"["#ffffff"]"##] {
            assert!(
                serde_json::from_str::<ColorReaderV1>(json).is_err(),
                "{json}"
            );
        }
    }
}
