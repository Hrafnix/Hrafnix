use crate::graphics::writer::DimensionWriter;
use crate::gui_metadata::data::GuiMetadata;
use serde::Serialize;

/// The current JSON representation of GUI metadata used when writing.
#[derive(Debug, Serialize)]
pub(crate) struct GuiMetadataWriter {
    /// Dimensions in drawing coordinates.
    dimensions: DimensionWriter,
    /// Icon path stored unchanged.
    icon: String,
}

impl From<&GuiMetadata> for GuiMetadataWriter {
    fn from(metadata: &GuiMetadata) -> Self {
        Self {
            dimensions: DimensionWriter::from(&metadata.dimensions),
            icon: metadata.icon.as_str().to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GuiMetadataWriter;
    use crate::common::deserialize::RootObject;
    use crate::graphics::data::Dimension;
    use crate::gui_metadata::data::GuiMetadata;
    use crate::gui_metadata::reader::v1::GuiMetadataReaderV1;
    use serde_json::{Value, json};

    #[test]
    fn metadata_matches_the_nested_reader_schema() {
        for (width, height) in [(100, 50), (0, u16::MAX), (u16::MAX, 0)] {
            for path in [
                "",
                "../icons/missing \u{3b1}.svg",
                " leading and trailing spaces ",
                r"C:\icons\missing.png",
            ] {
                let metadata = GuiMetadata {
                    dimensions: Dimension::new(width, height),
                    icon: path.into(),
                };
                let json = serde_json::to_string(&GuiMetadataWriter::from(&metadata)).unwrap();
                assert_eq!(
                    serde_json::from_str::<Value>(&json).unwrap(),
                    json!({
                        "dimensions": {"width": width, "height": height},
                        "icon": path,
                    }),
                );
                let reader: RootObject<GuiMetadataReaderV1> = serde_json::from_str(&json).unwrap();
                assert_eq!(GuiMetadata::from(reader.document), metadata);
            }
        }
    }
}
