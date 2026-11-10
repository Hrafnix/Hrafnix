use crate::background_decoration::data::{
    BackgroundDecoration, BackgroundDecorationKind, FrameStyle, RectangleStyle,
    RoundedRectangleStyle,
};
use crate::common::serialize::serialize_shareable_string;
use crate::graphics::writer::{
    ColorWriter, CornerRadiusWriter, DimensionWriter, PositionWriter, StrokeWidthWriter,
};
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// The current JSON representation of a background decoration used when writing.
#[derive(Debug, Serialize)]
pub(crate) struct BackgroundDecorationWriter {
    /// Identifier of the decoration.
    #[serde(serialize_with = "serialize_shareable_string")]
    id: ShareableString,
    /// Position.
    position: PositionWriter,
    /// Dimensions.
    dimensions: DimensionWriter,
    /// The shape and its applicable style.
    kind: BackgroundDecorationKindWriter,
}

impl From<&BackgroundDecoration> for BackgroundDecorationWriter {
    fn from(decoration: &BackgroundDecoration) -> Self {
        Self {
            id: decoration.id.clone(),
            position: PositionWriter::from(&decoration.position),
            dimensions: DimensionWriter::from(&decoration.dimensions),
            kind: BackgroundDecorationKindWriter::from(&decoration.kind),
        }
    }
}

/// The current JSON representation of background shape variants.
#[derive(Debug, Serialize)]
enum BackgroundDecorationKindWriter {
    /// A rectangular shape.
    Rectangle(RectangleStyleWriter),
    /// A rectangle with rounded corners.
    RoundedRectangle(RoundedRectangleStyleWriter),
    /// A rectangular outline.
    Frame(FrameStyleWriter),
}

impl From<&BackgroundDecorationKind> for BackgroundDecorationKindWriter {
    fn from(kind: &BackgroundDecorationKind) -> Self {
        match kind {
            BackgroundDecorationKind::Rectangle(style) => Self::Rectangle(style.into()),
            BackgroundDecorationKind::RoundedRectangle(style) => {
                Self::RoundedRectangle(style.into())
            }
            BackgroundDecorationKind::Frame(style) => Self::Frame(style.into()),
        }
    }
}

/// The current JSON representation of rectangle styling.
#[derive(Debug, Serialize)]
struct RectangleStyleWriter {
    /// Fill color.
    fill_color: ColorWriter,
    /// Outline color.
    stroke_color: ColorWriter,
    /// Outline width in stored hundredths.
    stroke_width: StrokeWidthWriter,
}

impl From<&RectangleStyle> for RectangleStyleWriter {
    fn from(style: &RectangleStyle) -> Self {
        Self {
            fill_color: ColorWriter::from(&style.fill_color),
            stroke_color: ColorWriter::from(&style.stroke_color),
            stroke_width: StrokeWidthWriter::from(&style.stroke_width),
        }
    }
}

/// The current JSON representation of rounded rectangle styling.
#[derive(Debug, Serialize)]
struct RoundedRectangleStyleWriter {
    /// Fill color.
    fill_color: ColorWriter,
    /// Outline color.
    stroke_color: ColorWriter,
    /// Outline width in stored hundredths.
    stroke_width: StrokeWidthWriter,
    /// Corner radius in stored hundredths.
    corner_radius: CornerRadiusWriter,
}

impl From<&RoundedRectangleStyle> for RoundedRectangleStyleWriter {
    fn from(style: &RoundedRectangleStyle) -> Self {
        Self {
            fill_color: ColorWriter::from(&style.fill_color),
            stroke_color: ColorWriter::from(&style.stroke_color),
            stroke_width: StrokeWidthWriter::from(&style.stroke_width),
            corner_radius: CornerRadiusWriter::from(&style.corner_radius),
        }
    }
}

/// The current JSON representation of frame styling.
#[derive(Debug, Serialize)]
struct FrameStyleWriter {
    /// Outline color.
    stroke_color: ColorWriter,
    /// Outline width in stored hundredths.
    stroke_width: StrokeWidthWriter,
}

impl From<&FrameStyle> for FrameStyleWriter {
    fn from(style: &FrameStyle) -> Self {
        Self {
            stroke_color: ColorWriter::from(&style.stroke_color),
            stroke_width: StrokeWidthWriter::from(&style.stroke_width),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BackgroundDecorationWriter;
    use crate::background_decoration::data::{
        BackgroundDecoration, BackgroundDecorationKind, FrameStyle, RectangleStyle,
        RoundedRectangleStyle,
    };
    use crate::background_decoration::reader::v1::BackgroundDecorationReaderV1;
    use crate::common::deserialize::RootObject;
    use crate::graphics::data::{Color, CornerRadius, Dimension, Position, Rgba, StrokeWidth};
    use serde_json::json;

    #[test]
    fn serializes_all_shapes_using_the_nested_reader_schema() {
        for (kind, tag, style) in [
            (
                BackgroundDecorationKind::Rectangle(RectangleStyle {
                    fill_color: Color::ThemeDefault,
                    stroke_color: Color::Custom(Rgba::new(0, 0, 0, 255)),
                    stroke_width: StrokeWidth::new(100),
                }),
                "Rectangle",
                json!({
                    "fill_color": "theme-default",
                    "stroke_color": "#000000",
                    "stroke_width": 100,
                }),
            ),
            (
                BackgroundDecorationKind::RoundedRectangle(RoundedRectangleStyle {
                    fill_color: Color::Custom(Rgba::new(255, 255, 255, 128)),
                    stroke_color: Color::ThemeDefault,
                    stroke_width: StrokeWidth::new(125),
                    corner_radius: CornerRadius::new(200),
                }),
                "RoundedRectangle",
                json!({
                    "fill_color": "#ffffff80",
                    "stroke_color": "theme-default",
                    "stroke_width": 125,
                    "corner_radius": 200,
                }),
            ),
            (
                BackgroundDecorationKind::Frame(FrameStyle {
                    stroke_color: Color::Custom(Rgba::new(170, 187, 204, 0)),
                    stroke_width: StrokeWidth::new(0),
                }),
                "Frame",
                json!({
                    "stroke_color": "#aabbcc00",
                    "stroke_width": 0,
                }),
            ),
        ] {
            let decoration = BackgroundDecoration {
                id: "shape \u{3b1}".into(),
                position: Position::new(-10, 20),
                dimensions: Dimension::new(100, 50),
                kind,
            };
            let original = decoration.clone();
            let written =
                serde_json::to_value(BackgroundDecorationWriter::from(&decoration)).unwrap();
            let mut expected = json!({
                "id": "shape \u{3b1}",
                "position": {"x": -10, "y": 20},
                "dimensions": {"width": 100, "height": 50},
                "kind": {},
            });
            expected["kind"][tag] = style;
            assert_eq!(written, expected);
            let reader: RootObject<BackgroundDecorationReaderV1> =
                serde_json::from_str(&serde_json::to_string(&written).unwrap()).unwrap();
            assert_eq!(BackgroundDecoration::from(reader.document), original);
            assert_eq!(decoration, original);
        }
    }

    #[test]
    fn writes_hundredths_at_both_numeric_bounds() {
        for value in [0, u16::MAX] {
            let decoration = BackgroundDecoration {
                id: "bounds".into(),
                position: Position::new(i32::MIN, i32::MAX),
                dimensions: Dimension::new(0, u16::MAX),
                kind: BackgroundDecorationKind::RoundedRectangle(RoundedRectangleStyle {
                    fill_color: Color::ThemeDefault,
                    stroke_color: Color::ThemeDefault,
                    stroke_width: StrokeWidth::new(value),
                    corner_radius: CornerRadius::new(value),
                }),
            };
            let written =
                serde_json::to_value(BackgroundDecorationWriter::from(&decoration)).unwrap();
            let style = &written["kind"]["RoundedRectangle"];
            assert_eq!(style["stroke_width"], json!(value));
            assert_eq!(style["corner_radius"], json!(value));
            assert_eq!(written["position"], json!({"x": i32::MIN, "y": i32::MAX}));
            assert_eq!(
                written["dimensions"],
                json!({"width": 0, "height": u16::MAX})
            );
        }
    }
}
