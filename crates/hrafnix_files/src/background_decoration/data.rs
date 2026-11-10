use crate::graphics::data::{Color, CornerRadius, Dimension, Position, StrokeWidth};
use hrafnix_shareable_string::ShareableString;

/// A shape behind a model's or component's contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackgroundDecoration {
    /// Identifier of the decoration.
    pub id: ShareableString,
    /// Position in drawing coordinates.
    pub position: Position,
    /// Dimensions in drawing coordinates.
    pub dimensions: Dimension,
    /// The shape of the decoration, including its type-specific style.
    pub kind: BackgroundDecorationKind,
}

/// The shapes allowed in a background layer, each with its own required style.
///
/// Styles cannot be assigned to a different shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundDecorationKind {
    /// A rectangular shape.
    Rectangle(RectangleStyle),
    /// A rectangle with rounded corners.
    RoundedRectangle(RoundedRectangleStyle),
    /// A rectangular outline.
    Frame(FrameStyle),
}

/// Fill and outline styling for a rectangle.
///
/// Every property is required. Outline width is
/// stored in hundredths of a drawing unit: `100` represents `1.00`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RectangleStyle {
    /// Fill color, using the theme's fill color for [`Color::ThemeDefault`].
    pub fill_color: Color,
    /// Outline color, using the theme's outline color for [`Color::ThemeDefault`].
    pub stroke_color: Color,
    /// Outline width in hundredths of a drawing unit.
    pub stroke_width: StrokeWidth,
}

/// Fill, outline, and corner styling for a rounded rectangle.
///
/// Every property is required. Numeric properties
/// are stored in hundredths of a drawing unit: `100` represents `1.00`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundedRectangleStyle {
    /// Fill color, using the theme's fill color for [`Color::ThemeDefault`].
    pub fill_color: Color,
    /// Outline color, using the theme's outline color for [`Color::ThemeDefault`].
    pub stroke_color: Color,
    /// Outline width in hundredths of a drawing unit.
    pub stroke_width: StrokeWidth,
    /// Corner radius in hundredths of a drawing unit.
    pub corner_radius: CornerRadius,
}

/// Outline styling for a frame.
///
/// Every property is required. Outline width is
/// stored in hundredths of a drawing unit: `100` represents `1.00`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameStyle {
    /// Outline color, using the theme's outline color for [`Color::ThemeDefault`].
    pub stroke_color: Color,
    /// Outline width in hundredths of a drawing unit.
    pub stroke_width: StrokeWidth,
}
