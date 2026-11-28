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
    pub(crate) const fn new(x: i32, y: i32) -> Self {
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
    pub(crate) const fn new(width: u16, height: u16) -> Self {
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

/// A clockwise quarter-turn rotation about the center of unrotated drawing bounds.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Rotation {
    /// No rotation.
    #[default]
    Degrees0 = 0,
    /// A 90-degree rotation.
    Degrees90 = 90,
    /// A 180-degree rotation.
    Degrees180 = 180,
    /// A 270-degree rotation.
    Degrees270 = 270,
}

impl Rotation {
    /// Returns the rotation in degrees.
    #[must_use]
    pub const fn degrees(self) -> u16 {
        match self {
            Self::Degrees0 => 0,
            Self::Degrees90 => 90,
            Self::Degrees180 => 180,
            Self::Degrees270 => 270,
        }
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
    pub(crate) const fn new(size: u16) -> Self {
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

#[cfg(test)]
mod tests {
    use super::FontSize;
    use std::ops::Div;

    #[test]
    fn hundredths_accessors() {
        for value in [0, 1234, u16::MAX] {
            let mut font_size = FontSize::new(value);
            assert_eq!(font_size.size(), value);
            let expected = Div::div(f32::from(value), 100.0).to_bits();
            assert_eq!(font_size.to_float().to_bits(), expected);
            font_size.set_size(4321);
            assert_eq!(font_size.size(), 4321);
        }
    }

    #[test]
    fn decimal_values_are_rounded_and_clamped_to_hundredths() {
        for (value, expected) in [
            (0.0, 0),
            (12.34, 1234),
            (12.344, 1234),
            (12.345, 1235),
            (655.35, u16::MAX),
            (655.36, u16::MAX),
            (-1.0, 0),
            (f32::NAN, 0),
            (f32::NEG_INFINITY, 0),
            (f32::INFINITY, u16::MAX),
        ] {
            let mut font_size = FontSize::new(1);
            font_size.set_from_float(value);
            assert_eq!(font_size.size(), expected, "FontSize from {value}");
        }
    }
}
