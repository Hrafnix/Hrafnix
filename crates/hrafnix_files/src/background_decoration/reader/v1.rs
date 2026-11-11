use crate::background_decoration::data::BackgroundDecoration;
use crate::common::deserialize::{deserialize_object, deserialize_unit_variant};
use crate::graphics::reader::v1::{DimensionReaderV1, PositionReaderV1};
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
        Self {
            id: decoration.id.into(),
            position: decoration.position.into(),
            dimensions: decoration.dimensions.into(),
            kind: decoration.kind.into(),
        }
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
    for crate::background_decoration::data::BackgroundDecorationKind
{
    fn from(kind: BackgroundDecorationKindReaderV1) -> Self {
        match kind {
            BackgroundDecorationKindReaderV1::Frame => Self::Frame,
        }
    }
}
