//! Tests for graphical styling values and colors.

use hrafnix_files::graphics::data::{Color, CornerRadius, FontSize, Rgba, StrokeWidth};
use std::ops::Div;

#[test]
fn hundredths_accessors() {
    for value in [0, 1234, u16::MAX] {
        let mut font_size = FontSize::new(value);
        let mut stroke_width = StrokeWidth::new(value);
        let mut corner_radius = CornerRadius::new(value);

        assert_eq!(font_size.size(), value);
        assert_eq!(stroke_width.width(), value);
        assert_eq!(corner_radius.radius(), value);

        let expected = Div::div(f32::from(value), 100.0).to_bits();
        assert_eq!(font_size.to_float().to_bits(), expected);
        assert_eq!(stroke_width.to_float().to_bits(), expected);
        assert_eq!(corner_radius.to_float().to_bits(), expected);

        font_size.set_size(4321);
        stroke_width.set_width(4321);
        corner_radius.set_radius(4321);

        assert_eq!(font_size.size(), 4321);
        assert_eq!(stroke_width.width(), 4321);
        assert_eq!(corner_radius.radius(), 4321);
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
        let mut stroke_width = StrokeWidth::new(1);
        let mut corner_radius = CornerRadius::new(1);

        font_size.set_from_float(value);
        stroke_width.set_from_float(value);
        corner_radius.set_from_float(value);

        assert_eq!(font_size.size(), expected, "FontSize from {value}");
        assert_eq!(stroke_width.width(), expected, "StrokeWidth from {value}");
        assert_eq!(
            corner_radius.radius(),
            expected,
            "CornerRadius from {value}"
        );
    }
}

#[test]
fn rgba_channels_can_be_read_and_updated_independently() {
    const COLOR: Rgba = Rgba::new(1, 2, 3, 4);
    let mut color = COLOR;
    assert_eq!(color.red(), 1);
    assert_eq!(color.green(), 2);
    assert_eq!(color.blue(), 3);
    assert_eq!(color.alpha(), 4);

    color.set_red(255);
    assert_eq!(color, Rgba::new(255, 2, 3, 4));
    color.set_green(0);
    assert_eq!(color, Rgba::new(255, 0, 3, 4));
    color.set_blue(255);
    assert_eq!(color, Rgba::new(255, 0, 255, 4));
    color.set_alpha(0);
    assert_eq!(color, Rgba::new(255, 0, 255, 0));
    color.set_alpha(255);
    assert_eq!(color, Rgba::new(255, 0, 255, 255));
    assert_eq!(COLOR, Rgba::new(1, 2, 3, 4));
}

#[test]
fn theme_default_and_custom_transparent_colors_are_distinct() {
    assert_ne!(Color::ThemeDefault, Color::Custom(Rgba::new(0, 0, 0, 0)));
}
