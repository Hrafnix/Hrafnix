use crate::data::graphics::{Dimension, FontSize, Position, Rotation};
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

/// The version 1 JSON representation of an unsigned integer rotation.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub(crate) struct RotationReaderV1 {
    /// Clockwise degrees, rounded to the nearest quarter turn during conversion.
    degrees: u16,
}

impl From<RotationReaderV1> for Rotation {
    fn from(rotation: RotationReaderV1) -> Self {
        match rotation.degrees % 360 {
            45..=134 => Self::Degrees90,
            135..=224 => Self::Degrees180,
            225..=314 => Self::Degrees270,
            _ => Self::Degrees0,
        }
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
    use super::{DimensionReaderV1, FontSizeReaderV1, PositionReaderV1, RotationReaderV1};
    use crate::common::deserialize::RootObject;
    use crate::data::graphics::{Dimension, FontSize, Position, Rotation};
    use serde_json::json;

    #[test]
    fn reads_positions_and_dimensions_at_numeric_bounds() {
        let position: RootObject<PositionReaderV1> =
            serde_json::from_value(json!({"x": i32::MIN, "y": i32::MAX})).unwrap();
        assert_eq!(
            Position::from(position.document),
            Position::new(i32::MIN, i32::MAX)
        );
        let dimensions: RootObject<DimensionReaderV1> =
            serde_json::from_value(json!({"width": 0, "height": u16::MAX})).unwrap();
        assert_eq!(
            Dimension::from(dimensions.document),
            Dimension::new(0, u16::MAX)
        );
    }

    #[test]
    fn geometry_rejects_missing_out_of_range_and_unknown_fields() {
        for invalid in [
            json!({"x": 0}),
            json!({"x": 2147483648_u64, "y": 0}),
            json!({"x": 0, "y": -2147483649_i64}),
            json!({"x": 0, "y": 0, "extra": 1}),
            json!([0, 0]),
        ] {
            assert!(serde_json::from_value::<RootObject<PositionReaderV1>>(invalid).is_err());
        }
        for invalid in [
            json!({"width": 0}),
            json!({"width": -1, "height": 0}),
            json!({"width": 0, "height": 65536}),
            json!({"width": 0, "height": 0, "extra": 1}),
            json!([0, 0]),
        ] {
            assert!(serde_json::from_value::<RootObject<DimensionReaderV1>>(invalid).is_err());
        }
    }

    #[test]
    fn rotations_are_plain_unsigned_integers() {
        assert_eq!(Rotation::default(), Rotation::Degrees0);
        for (json, expected) in [
            ("0", Rotation::Degrees0),
            ("90", Rotation::Degrees90),
            ("180", Rotation::Degrees180),
            ("270", Rotation::Degrees270),
        ] {
            let reader: RotationReaderV1 = serde_json::from_str(json).unwrap();
            let rotation = Rotation::from(reader);
            assert_eq!(rotation, expected);
            assert_eq!(rotation.degrees().to_string(), json);
        }

        for json in [
            "-90",
            "65536",
            "90.0",
            "9e1",
            "0.5",
            "\"90\"",
            "\"Degrees90\"",
            "true",
            "null",
            "[90]",
            r#"{"degrees": 90}"#,
        ] {
            assert!(
                serde_json::from_str::<RotationReaderV1>(json).is_err(),
                "{json}"
            );
        }
    }

    #[test]
    fn all_unsigned_rotations_round_to_the_nearest_quarter_turn() {
        for degrees in 0..=u16::MAX {
            let reader: RotationReaderV1 = serde_json::from_str(&degrees.to_string()).unwrap();
            let expected = ((u32::from(degrees) + 45) / 90 % 4) * 90;
            assert_eq!(
                u32::from(Rotation::from(reader).degrees()),
                expected,
                "{degrees}"
            );
        }
    }

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
