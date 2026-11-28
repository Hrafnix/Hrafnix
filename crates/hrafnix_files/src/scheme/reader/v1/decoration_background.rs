use crate::common::deserialize::{deserialize_object, deserialize_unit_variant};
use crate::data::decoration_background::BackgroundDecoration;
use crate::scheme::reader::v1::graphics::{DimensionReaderV1, PositionReaderV1};
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
    /// The shape of the decoration, written as a plain string.
    #[serde(deserialize_with = "deserialize_unit_variant")]
    pub kind: BackgroundDecorationKindReaderV1,
}

impl From<BackgroundDecorationReaderV1> for BackgroundDecoration {
    fn from(decoration: BackgroundDecorationReaderV1) -> Self {
        Self::new(
            decoration.id.into(),
            decoration.position.into(),
            decoration.dimensions.into(),
            decoration.kind.into(),
        )
    }
}

/// The shapes allowed in a background layer.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) enum BackgroundDecorationKindReaderV1 {
    /// A rectangular outline.
    Frame,
}

impl From<BackgroundDecorationKindReaderV1>
    for crate::data::decoration_background::BackgroundDecorationKind
{
    fn from(kind: BackgroundDecorationKindReaderV1) -> Self {
        match kind {
            BackgroundDecorationKindReaderV1::Frame => Self::Frame,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BackgroundDecorationReaderV1;
    use crate::common::deserialize::RootObject;
    use crate::data::decoration_background::{BackgroundDecoration, BackgroundDecorationKind};
    use crate::data::graphics::{Dimension, Position};
    use serde_json::json;

    #[test]
    fn reads_frame_and_preserves_geometry() {
        let reader: RootObject<BackgroundDecorationReaderV1> = serde_json::from_value(json!({
            "id": "frame", "position": {"x": -10, "y": 20},
            "dimensions": {"width": 100, "height": 50}, "kind": "Frame",
        }))
        .unwrap();
        assert_eq!(
            BackgroundDecoration::from(reader.document),
            BackgroundDecoration::new(
                "frame".into(),
                Position::new(-10, 20),
                Dimension::new(100, 50),
                BackgroundDecorationKind::Frame,
            ),
        );
    }

    #[test]
    fn kind_requires_a_known_plain_string() {
        for kind in [
            json!("Unknown"),
            json!({"Frame": null}),
            json!(0),
            json!(null),
        ] {
            assert!(
                serde_json::from_value::<RootObject<BackgroundDecorationReaderV1>>(json!({
                    "id": "frame", "position": {"x": 0, "y": 0},
                    "dimensions": {"width": 100, "height": 50}, "kind": kind,
                }))
                .is_err()
            );
        }
    }
}
