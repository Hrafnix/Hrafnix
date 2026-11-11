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
