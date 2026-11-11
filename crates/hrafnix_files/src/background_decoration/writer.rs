use crate::background_decoration::data::{BackgroundDecoration, BackgroundDecorationKind};
use crate::common::serialize::serialize_shareable_string;
use crate::graphics::writer::{DimensionWriter, PositionWriter};
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
    /// The shape.
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
    /// A rectangular outline.
    Frame,
}

impl From<&BackgroundDecorationKind> for BackgroundDecorationKindWriter {
    fn from(kind: &BackgroundDecorationKind) -> Self {
        match kind {
            BackgroundDecorationKind::Frame => Self::Frame,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BackgroundDecorationWriter;
    use crate::background_decoration::data::{BackgroundDecoration, BackgroundDecorationKind};
    use crate::background_decoration::reader::v1::BackgroundDecorationReaderV1;
    use crate::common::deserialize::RootObject;
    use crate::graphics::data::{Dimension, Position};
    use serde_json::json;

    #[test]
    fn serializes_frame_using_the_reader_schema() {
        let decoration = BackgroundDecoration {
            id: "shape \u{3b1}".into(),
            position: Position::new(-10, 20),
            dimensions: Dimension::new(100, 50),
            kind: BackgroundDecorationKind::Frame,
        };
        let original = decoration.clone();
        let written = serde_json::to_value(BackgroundDecorationWriter::from(&decoration)).unwrap();
        assert_eq!(
            written,
            json!({
                "id": "shape \u{3b1}",
                "position": {"x": -10, "y": 20},
                "dimensions": {"width": 100, "height": 50},
                "kind": "Frame",
            })
        );
        let reader: RootObject<BackgroundDecorationReaderV1> =
            serde_json::from_str(&serde_json::to_string(&written).unwrap()).unwrap();
        assert_eq!(BackgroundDecoration::from(reader.document), original);
        assert_eq!(decoration, original);
    }

    #[test]
    fn writes_geometry_at_both_numeric_bounds() {
        let decoration = BackgroundDecoration {
            id: "bounds".into(),
            position: Position::new(i32::MIN, i32::MAX),
            dimensions: Dimension::new(0, u16::MAX),
            kind: BackgroundDecorationKind::Frame,
        };
        let written = serde_json::to_value(BackgroundDecorationWriter::from(&decoration)).unwrap();
        assert_eq!(written["position"], json!({"x": i32::MIN, "y": i32::MAX}));
        assert_eq!(
            written["dimensions"],
            json!({"width": 0, "height": u16::MAX})
        );
    }
}
