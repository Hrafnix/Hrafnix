use crate::common::serialize::serialize_shareable_string;
use crate::foreground_decoration::data::{
    ForegroundDecoration, ForegroundDecorationKind, ImageDecoration, ImageFit, TextDecoration,
};
use crate::graphics::writer::{ColorWriter, DimensionWriter, FontSizeWriter, PositionWriter};
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// The current JSON representation of a foreground decoration used when writing.
#[derive(Debug, Serialize)]
pub(crate) struct ForegroundDecorationWriter {
    /// Identifier of the decoration.
    #[serde(serialize_with = "serialize_shareable_string")]
    id: ShareableString,
    /// Position.
    position: PositionWriter,
    /// Dimensions.
    dimensions: DimensionWriter,
    /// The applicable text or image payload.
    kind: ForegroundDecorationKindWriter,
}

impl From<&ForegroundDecoration> for ForegroundDecorationWriter {
    fn from(decoration: &ForegroundDecoration) -> Self {
        Self {
            id: decoration.id.clone(),
            position: PositionWriter::from(&decoration.position),
            dimensions: DimensionWriter::from(&decoration.dimensions),
            kind: ForegroundDecorationKindWriter::from(&decoration.kind),
        }
    }
}

/// The current JSON representation of foreground content variants.
#[derive(Debug, Serialize)]
enum ForegroundDecorationKindWriter {
    /// Text content.
    Text(TextDecorationWriter),
    /// Image content and fitting mode.
    Image(ImageDecorationWriter),
}

impl From<&ForegroundDecorationKind> for ForegroundDecorationKindWriter {
    fn from(kind: &ForegroundDecorationKind) -> Self {
        match kind {
            ForegroundDecorationKind::Text(text) => Self::Text(text.into()),
            ForegroundDecorationKind::Image(image) => Self::Image(image.into()),
        }
    }
}
/// The current JSON representation of text content and styling.
#[derive(Debug, Serialize)]
struct TextDecorationWriter {
    /// Displayed text.
    #[serde(serialize_with = "serialize_shareable_string")]
    text: ShareableString,
    /// Font size in stored hundredths.
    font_size: FontSizeWriter,
    /// Text color.
    text_color: ColorWriter,
}

impl From<&TextDecoration> for TextDecorationWriter {
    fn from(text: &TextDecoration) -> Self {
        Self {
            text: text.text.clone(),
            font_size: FontSizeWriter::from(&text.font_size),
            text_color: ColorWriter::from(&text.text_color),
        }
    }
}

/// The current JSON representation of an image source and fitting mode.
#[derive(Debug, Serialize)]
struct ImageDecorationWriter {
    /// Path stored unchanged.
    path: String,
    /// Fitting mode.
    fit: ImageFitWriter,
}

impl From<&ImageDecoration> for ImageDecorationWriter {
    fn from(image: &ImageDecoration) -> Self {
        Self {
            path: image.path.as_str().to_owned(),
            fit: ImageFitWriter::from(image.fit),
        }
    }
}

/// The current JSON representation of image fitting modes.
#[derive(Debug, Serialize)]
enum ImageFitWriter {
    /// Show the whole image.
    Contain,
    /// Cover the bounds.
    Cover,
    /// Stretch to the bounds.
    Fill,
}

impl From<ImageFit> for ImageFitWriter {
    fn from(fit: ImageFit) -> Self {
        match fit {
            ImageFit::Contain => Self::Contain,
            ImageFit::Cover => Self::Cover,
            ImageFit::Fill => Self::Fill,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ForegroundDecorationWriter;
    use crate::common::deserialize::RootObject;
    use crate::foreground_decoration::data::{
        ForegroundDecoration, ForegroundDecorationKind, ImageDecoration, ImageFit, TextDecoration,
    };
    use crate::foreground_decoration::reader::v1::ForegroundDecorationReaderV1;
    use crate::graphics::data::{Color, Dimension, FontSize, Position, Rgba};
    use serde_json::{Value, json};

    /// Checks the exact wire format and reads it back through the independent reader.
    fn assert_round_trip(decoration: &ForegroundDecoration, expected: &Value) {
        let json = serde_json::to_string(&ForegroundDecorationWriter::from(decoration)).unwrap();
        assert_eq!(serde_json::from_str::<Value>(&json).unwrap(), *expected);
        let reader: RootObject<ForegroundDecorationReaderV1> = serde_json::from_str(&json).unwrap();
        assert_eq!(ForegroundDecoration::from(reader.document), *decoration);
    }

    #[test]
    fn text_matches_the_nested_reader_schema() {
        for (color, encoded) in [
            (Color::ThemeDefault, "theme-default"),
            (Color::Custom(Rgba::new(0, 0, 0, 255)), "#000000"),
            (Color::Custom(Rgba::new(170, 187, 204, 128)), "#aabbcc80"),
            (Color::Custom(Rgba::new(170, 187, 204, 0)), "#aabbcc00"),
        ] {
            let decoration = ForegroundDecoration {
                id: "label".into(),
                position: Position::new(-10, 20),
                dimensions: Dimension::new(100, 50),
                kind: ForegroundDecorationKind::Text(TextDecoration {
                    text: "Title \u{3b1}".into(),
                    font_size: FontSize::new(2050),
                    text_color: color,
                }),
            };
            assert_round_trip(
                &decoration,
                &json!({
                    "id": "label",
                    "position": {"x": -10, "y": 20},
                    "dimensions": {"width": 100, "height": 50},
                    "kind": {
                        "Text": {
                            "text": "Title \u{3b1}",
                            "font_size": 2050,
                            "text_color": encoded,
                        },
                    },
                }),
            );
        }
    }

    #[test]
    fn images_match_the_nested_reader_schema() {
        for (fit, encoded) in [
            (ImageFit::Contain, "Contain"),
            (ImageFit::Cover, "Cover"),
            (ImageFit::Fill, "Fill"),
        ] {
            let decoration = ForegroundDecoration {
                id: "image".into(),
                position: Position::new(-10, -20),
                dimensions: Dimension::new(100, 50),
                kind: ForegroundDecorationKind::Image(ImageDecoration {
                    path: "../images/missing \u{3b1}.svg".into(),
                    fit,
                }),
            };
            assert_round_trip(
                &decoration,
                &json!({
                    "id": "image",
                    "position": {"x": -10, "y": -20},
                    "dimensions": {"width": 100, "height": 50},
                    "kind": {
                        "Image": {
                            "path": "../images/missing \u{3b1}.svg",
                            "fit": encoded,
                        },
                    },
                }),
            );
        }
    }
}
