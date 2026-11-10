use std::ops::{Div, Mul};

/// A position in drawing coordinates.
///
/// All fields are signed 32-bit integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// Horizontal position.
    x: i32,
    /// Vertical position.
    y: i32,
}

impl Position {
    /// Creates a new position with the given coordinates.
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns the horizontal position.
    #[must_use]
    pub const fn x(&self) -> i32 {
        self.x
    }

    /// Returns the vertical position.
    #[must_use]
    pub const fn y(&self) -> i32 {
        self.y
    }

    /// Sets the horizontal position.
    pub const fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    /// Sets the vertical position.
    pub const fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

/// Dimensions in drawing coordinates.
///
/// All fields are unsigned 16-bit integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimension {
    /// Width in drawing coordinates.
    width: u16,
    /// Height in drawing coordinates.
    height: u16,
}

impl Dimension {
    /// Creates new dimensions with the given width and height.
    #[must_use]
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Returns the width.
    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width
    }

    /// Returns the height.
    #[must_use]
    pub const fn height(&self) -> u16 {
        self.height
    }

    /// Sets the width.
    pub const fn set_width(&mut self, width: u16) {
        self.width = width;
    }

    /// Sets the height.
    pub const fn set_height(&mut self, height: u16) {
        self.height = height;
    }
}

/// A font size stored in hundredths (1234 means 12.34).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontSize {
    /// Size in hundredths.
    size: u16,
}

impl FontSize {
    /// Creates a new font size with the given size.
    #[must_use]
    pub const fn new(size: u16) -> Self {
        Self { size }
    }

    /// Returns the size.
    #[must_use]
    pub const fn size(&self) -> u16 {
        self.size
    }

    /// Sets the size.
    pub const fn set_size(&mut self, size: u16) {
        self.size = size;
    }

    /// Returns the size as a decimal value.
    #[must_use]
    pub fn to_float(&self) -> f32 {
        Div::div(f32::from(self.size), 100.0)
    }

    /// Sets the font size from a decimal value, rounded to hundredths and
    /// clamped to the `u16` range. NaN converts to zero.
    pub fn set_from_float(&mut self, value: f32) {
        self.size = hundredths_from_float(value);
    }
}

/// A stroke width stored in hundredths (1234 means 12.34).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrokeWidth {
    /// Width in hundredths.
    width: u16,
}

impl StrokeWidth {
    /// Creates a new stroke width with the given width.
    #[must_use]
    pub const fn new(width: u16) -> Self {
        Self { width }
    }

    /// Returns the width.
    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width
    }

    /// Sets the width.
    pub const fn set_width(&mut self, width: u16) {
        self.width = width;
    }

    /// Returns the width as a decimal value.
    #[must_use]
    pub fn to_float(&self) -> f32 {
        Div::div(f32::from(self.width), 100.0)
    }

    /// Sets the stroke width from a decimal value, rounded to hundredths and
    /// clamped to the `u16` range. NaN converts to zero.
    pub fn set_from_float(&mut self, value: f32) {
        self.width = hundredths_from_float(value);
    }
}

/// A corner radius stored in hundredths (1234 means 12.34).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CornerRadius {
    /// Radius in hundredths.
    radius: u16,
}

impl CornerRadius {
    /// Creates a new corner radius with the given radius.
    #[must_use]
    pub const fn new(radius: u16) -> Self {
        Self { radius }
    }

    /// Returns the radius.
    #[must_use]
    pub const fn radius(&self) -> u16 {
        self.radius
    }

    /// Sets the radius.
    pub const fn set_radius(&mut self, radius: u16) {
        self.radius = radius;
    }

    /// Returns the radius as a decimal value.
    #[must_use]
    pub fn to_float(&self) -> f32 {
        Div::div(f32::from(self.radius), 100.0)
    }

    /// Sets the corner radius from a decimal value, rounded to hundredths and
    /// clamped to the `u16` range. NaN converts to zero.
    pub fn set_from_float(&mut self, value: f32) {
        self.radius = hundredths_from_float(value);
    }
}

/// Rounds a decimal value to hundredths and clamps it to the `u16` range.
fn hundredths_from_float(value: f32) -> u16 {
    let value = Mul::mul(value, 100.0).round();

    if value.is_nan() || value <= 0.0 {
        return 0;
    }

    // Every u16 is exactly representable as a f32, so setting each bit from
    // the most significant down finds the largest u16 not above the value.
    (0..u16::BITS).rev().fold(0, |result, bit| {
        let candidate = result | (1 << bit);
        if f32::from(candidate) <= value {
            candidate
        } else {
            result
        }
    })
}

/// A theme-provided color or an explicit custom color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// The renderer's theme color for the role, such as text, fill, or outline.
    ThemeDefault,
    /// An exact color that does not change with the theme.
    Custom(Rgba),
}

/// An sRGB color with straight (unpremultiplied) alpha.
///
/// Channels range from 0 to 255. Alpha 0 is transparent and 255 is opaque.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    /// Red channel.
    red: u8,
    /// Green channel.
    green: u8,
    /// Blue channel.
    blue: u8,
    /// Alpha channel.
    alpha: u8,
}

impl Rgba {
    /// Creates a color with the given red, green, blue, and alpha channels.
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Returns the red channel.
    #[must_use]
    pub const fn red(&self) -> u8 {
        self.red
    }

    /// Returns the green channel.
    #[must_use]
    pub const fn green(&self) -> u8 {
        self.green
    }

    /// Returns the blue channel.
    #[must_use]
    pub const fn blue(&self) -> u8 {
        self.blue
    }

    /// Returns the alpha channel.
    #[must_use]
    pub const fn alpha(&self) -> u8 {
        self.alpha
    }

    /// Sets the red channel.
    pub const fn set_red(&mut self, red: u8) {
        self.red = red;
    }

    /// Sets the green channel.
    pub const fn set_green(&mut self, green: u8) {
        self.green = green;
    }

    /// Sets the blue channel.
    pub const fn set_blue(&mut self, blue: u8) {
        self.blue = blue;
    }

    /// Sets the alpha channel.
    pub const fn set_alpha(&mut self, alpha: u8) {
        self.alpha = alpha;
    }
}
