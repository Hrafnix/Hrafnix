use crate::graphics::data::{Dimension, FontSize, Position};
use serde::Deserialize;

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

#[cfg(test)]
mod tests {
    use super::FontSizeReaderV1;
    use crate::graphics::data::FontSize;

    #[test]
    fn hundredths_are_plain_integers() {
        let font_size: FontSizeReaderV1 = serde_json::from_str("1234").unwrap();
        assert_eq!(FontSize::from(font_size).size(), 1234);

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
        }
    }
}
