//! Tests for graphical styling values.

use hrafnix_files::graphics::data::FontSize;
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
