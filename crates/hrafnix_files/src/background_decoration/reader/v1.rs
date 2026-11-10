use crate::background_decoration::data::BackgroundDecoration;
use crate::common::deserialize::deserialize_object;
use crate::graphics::reader::v1::{
    ColorReaderV1, CornerRadiusReaderV1, DimensionReaderV1, PositionReaderV1, StrokeWidthReaderV1,
};
use serde::Deserialize;

/// A shape behind a model's or component's contents.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BackgroundDecorationReaderV1 {
    /// Identifier of the decoration.
    pub id: String,
    /// Position in drawing coordinates.
    #[serde(deserialize_with = "deserialize_object")]
    pub position: PositionReaderV1,
    /// Dimensions in drawing coordinates.
    #[serde(deserialize_with = "deserialize_object")]
    pub dimensions: DimensionReaderV1,
    /// The shape of the decoration, including its type-specific style.
    pub kind: BackgroundDecorationKindReaderV1,
}

impl From<BackgroundDecorationReaderV1> for BackgroundDecoration {
    fn from(decoration: BackgroundDecorationReaderV1) -> Self {
        Self {
            id: decoration.id.into(),
            position: decoration.position.into(),
            dimensions: decoration.dimensions.into(),
            kind: decoration.kind.into(),
        }
    }
}

/// The shapes allowed in a background layer, each with its own required style.
///
/// Styles cannot be assigned to a different shape.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) enum BackgroundDecorationKindReaderV1 {
    /// A rectangular shape.
    Rectangle(#[serde(deserialize_with = "deserialize_object")] RectangleStyleReaderV1),
    /// A rectangle with rounded corners.
    RoundedRectangle(
        #[serde(deserialize_with = "deserialize_object")] RoundedRectangleStyleReaderV1,
    ),
    /// A rectangular outline.
    Frame(#[serde(deserialize_with = "deserialize_object")] FrameStyleReaderV1),
}

impl From<BackgroundDecorationKindReaderV1>
    for crate::background_decoration::data::BackgroundDecorationKind
{
    fn from(kind: BackgroundDecorationKindReaderV1) -> Self {
        match kind {
            BackgroundDecorationKindReaderV1::Rectangle(style) => Self::Rectangle(style.into()),
            BackgroundDecorationKindReaderV1::RoundedRectangle(style) => {
                Self::RoundedRectangle(style.into())
            }
            BackgroundDecorationKindReaderV1::Frame(style) => Self::Frame(style.into()),
        }
    }
}

/// Fill and outline styling for a rectangle.
///
/// Every property is required. Outline width is
/// stored in hundredths of a drawing unit: `100` represents `1.00`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RectangleStyleReaderV1 {
    /// Fill color, using the theme's fill color for [`ThemeDefault`](crate::graphics::data::Color::ThemeDefault).
    pub fill_color: ColorReaderV1,
    /// Outline color, using the theme's outline color for [`ThemeDefault`](crate::graphics::data::Color::ThemeDefault).
    pub stroke_color: ColorReaderV1,
    /// Outline width in hundredths of a drawing unit.
    pub stroke_width: StrokeWidthReaderV1,
}

impl From<RectangleStyleReaderV1> for crate::background_decoration::data::RectangleStyle {
    fn from(style: RectangleStyleReaderV1) -> Self {
        Self {
            fill_color: style.fill_color.into(),
            stroke_color: style.stroke_color.into(),
            stroke_width: style.stroke_width.into(),
        }
    }
}

/// Fill, outline, and corner styling for a rounded rectangle.
///
/// Every property is required. Numeric properties
/// are stored in hundredths of a drawing unit: `100` represents `1.00`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RoundedRectangleStyleReaderV1 {
    /// Fill color, using the theme's fill color for [`ThemeDefault`](crate::graphics::data::Color::ThemeDefault).
    pub fill_color: ColorReaderV1,
    /// Outline color, using the theme's outline color for [`ThemeDefault`](crate::graphics::data::Color::ThemeDefault).
    pub stroke_color: ColorReaderV1,
    /// Outline width in hundredths of a drawing unit.
    pub stroke_width: StrokeWidthReaderV1,
    /// Corner radius in hundredths of a drawing unit.
    pub corner_radius: CornerRadiusReaderV1,
}

impl From<RoundedRectangleStyleReaderV1>
    for crate::background_decoration::data::RoundedRectangleStyle
{
    fn from(style: RoundedRectangleStyleReaderV1) -> Self {
        Self {
            fill_color: style.fill_color.into(),
            stroke_color: style.stroke_color.into(),
            stroke_width: style.stroke_width.into(),
            corner_radius: style.corner_radius.into(),
        }
    }
}

/// Outline styling for a frame.
///
/// Every property is required. Outline width is
/// stored in hundredths of a drawing unit: `100` represents `1.00`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FrameStyleReaderV1 {
    /// Outline color, using the theme's outline color for [`ThemeDefault`](crate::graphics::data::Color::ThemeDefault).
    pub stroke_color: ColorReaderV1,
    /// Outline width in hundredths of a drawing unit.
    pub stroke_width: StrokeWidthReaderV1,
}

impl From<FrameStyleReaderV1> for crate::background_decoration::data::FrameStyle {
    fn from(style: FrameStyleReaderV1) -> Self {
        Self {
            stroke_color: style.stroke_color.into(),
            stroke_width: style.stroke_width.into(),
        }
    }
}
