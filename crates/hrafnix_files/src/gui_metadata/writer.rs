use crate::common::serialize::serialize_port_key;
use crate::graphics::writer::DimensionWriter;
use crate::gui_metadata::data::{GuiMetadata, Port, PortLocation};
use hrafnix_common::port::PortType;
use hrafnix_keys::PortKey;
use serde::Serialize;

/// The current JSON representation of GUI metadata used when writing.
#[derive(Debug, Serialize)]
pub(crate) struct GuiMetadataWriter {
    /// Dimensions in drawing coordinates.
    dimensions: DimensionWriter,
    /// Icon path stored unchanged.
    icon: String,
    /// Ports with locations capped to the icon's drawing dimensions.
    ports: Vec<PortWriter>,
}

impl From<&GuiMetadata> for GuiMetadataWriter {
    fn from(metadata: &GuiMetadata) -> Self {
        Self {
            dimensions: DimensionWriter::from(&metadata.dimensions),
            icon: metadata.icon.as_str().to_owned(),
            ports: metadata
                .ports
                .iter()
                .map(|port| {
                    let mut port = PortWriter::from(port);
                    port.location.x = port.location.x.min(metadata.dimensions.width());
                    port.location.y = port.location.y.min(metadata.dimensions.height());
                    port
                })
                .collect(),
        }
    }
}

/// The current JSON representation of a GUI port used when writing.
#[derive(Debug, Serialize)]
struct PortWriter {
    /// Validated identifier of the port, represented by a plain string.
    #[serde(serialize_with = "serialize_port_key")]
    id: PortKey,
    /// The role of the connection.
    #[serde(rename = "type")]
    port_type: PortTypeWriter,
    /// Icon-local drawing coordinates.
    location: PortLocationWriter,
}

impl From<&Port> for PortWriter {
    fn from(port: &Port) -> Self {
        Self {
            id: port.id.clone(),
            port_type: port.port_type.into(),
            location: PortLocationWriter::from(&port.location),
        }
    }
}

/// The current JSON names for connection roles.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum PortTypeWriter {
    /// Receives a signal.
    Input,
    /// Emits a signal.
    Output,
    /// Exchanges physical power.
    Power,
    /// Exchanges signals in both directions.
    BidirectionalSignal,
    /// Exposes a subsystem connection.
    System,
    /// Contains multiple physical subports.
    PowerMulti,
    /// Contains multiple signal-input subports.
    InputMulti,
}

impl From<PortType> for PortTypeWriter {
    fn from(port_type: PortType) -> Self {
        match port_type {
            PortType::Input => Self::Input,
            PortType::Output => Self::Output,
            PortType::Power => Self::Power,
            PortType::BidirectionalSignal => Self::BidirectionalSignal,
            PortType::System => Self::System,
            PortType::PowerMulti => Self::PowerMulti,
            PortType::InputMulti => Self::InputMulti,
        }
    }
}

/// The current JSON representation of an icon-local location.
#[derive(Debug, Serialize)]
struct PortLocationWriter {
    /// Horizontal drawing coordinate.
    x: u16,
    /// Vertical drawing coordinate.
    y: u16,
}

impl From<&PortLocation> for PortLocationWriter {
    fn from(location: &PortLocation) -> Self {
        Self {
            x: location.x,
            y: location.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GuiMetadataWriter;
    use crate::common::deserialize::RootObject;
    use crate::graphics::data::Dimension;
    use crate::gui_metadata::data::{GuiMetadata, Port, PortLocation};
    use crate::gui_metadata::reader::v1::GuiMetadataReaderV1;
    use hrafnix_common::port::PortType;
    use hrafnix_keys::port_key;
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
                    ports: Vec::new(),
                };
                let json = serde_json::to_string(&GuiMetadataWriter::from(&metadata)).unwrap();
                assert_eq!(
                    serde_json::from_str::<Value>(&json).unwrap(),
                    json!({
                        "dimensions": {"width": width, "height": height},
                        "icon": path,
                        "ports": [],
                    }),
                );
                let reader: RootObject<GuiMetadataReaderV1> = serde_json::from_str(&json).unwrap();
                assert_eq!(GuiMetadata::from(reader.document), metadata);
            }
        }
    }

    #[test]
    fn all_port_types_and_boundary_locations_round_trip() {
        for (port_type, name) in [
            (PortType::Input, "input"),
            (PortType::Output, "output"),
            (PortType::Power, "power"),
            (PortType::BidirectionalSignal, "bidirectional-signal"),
            (PortType::System, "system"),
            (PortType::PowerMulti, "power-multi"),
            (PortType::InputMulti, "input-multi"),
        ] {
            for (width, height) in [(100, 50), (0, 0), (0, u16::MAX), (u16::MAX, 0)] {
                for (x, y) in [(0, 0), (width, height), (width, 0), (0, height)] {
                    let metadata = GuiMetadata {
                        dimensions: Dimension::new(width, height),
                        icon: "icon.svg".into(),
                        ports: vec![Port {
                            id: port_key!("temperature_out1").into(),
                            port_type,
                            location: PortLocation { x, y },
                        }],
                    };
                    let text = serde_json::to_string(&GuiMetadataWriter::from(&metadata)).unwrap();
                    assert_eq!(
                        serde_json::from_str::<Value>(&text).unwrap(),
                        json!({
                            "dimensions": {"width": width, "height": height},
                            "icon": "icon.svg",
                            "ports": [{
                                "id": "temperature_out1",
                                "type": name,
                                "location": {"x": x, "y": y},
                            }],
                        }),
                    );
                    let reader: RootObject<GuiMetadataReaderV1> =
                        serde_json::from_str(&text).unwrap();
                    assert_eq!(GuiMetadata::from(reader.document), metadata);
                }
            }
        }
    }

    #[test]
    fn writing_caps_ports_to_the_icon_bounds() {
        for (width, height) in [(100, 50), (0, 0), (0, u16::MAX), (u16::MAX, 0)] {
            for (x, y) in [
                (101, 25),
                (100, 51),
                (u16::MAX, 0),
                (0, u16::MAX),
                (u16::MAX, u16::MAX),
            ] {
                let metadata = GuiMetadata {
                    dimensions: Dimension::new(width, height),
                    icon: "".into(),
                    ports: vec![Port {
                        id: port_key!("temperature").into(),
                        port_type: PortType::Output,
                        location: PortLocation { x, y },
                    }],
                };
                let original = metadata.clone();
                let written = serde_json::to_value(GuiMetadataWriter::from(&metadata)).unwrap();
                assert_eq!(
                    written,
                    json!({
                        "dimensions": {"width": width, "height": height},
                        "icon": "",
                        "ports": [{
                            "id": "temperature",
                            "type": "output",
                            "location": {"x": x.min(width), "y": y.min(height)},
                        }],
                    }),
                );
                assert_eq!(metadata, original);
            }
        }
    }
}
