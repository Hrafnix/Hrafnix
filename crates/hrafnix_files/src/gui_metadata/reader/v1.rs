use crate::common::deserialize::{
    deserialize_object, deserialize_objects, deserialize_port_key, deserialize_unit_variant,
};
use crate::graphics::data::Dimension;
use crate::graphics::reader::v1::DimensionReaderV1;
use crate::gui_metadata::data::{GuiMetadata, Port, PortLocation};
use hrafnix_common::port::PortType;
use hrafnix_keys::PortKey;
use serde::Deserialize;

/// The version 1 JSON representation of GUI metadata.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GuiMetadataReaderV1 {
    /// Dimensions in drawing coordinates.
    #[serde(deserialize_with = "deserialize_object")]
    dimensions: DimensionReaderV1,
    /// Icon path, stored unchanged rather than opened or resolved during file I/O.
    icon: String,
    /// Ports placed within the icon's drawing dimensions.
    #[serde(deserialize_with = "deserialize_objects")]
    ports: Vec<PortReaderV1>,
}

impl From<GuiMetadataReaderV1> for GuiMetadata {
    fn from(metadata: GuiMetadataReaderV1) -> Self {
        let dimensions: Dimension = metadata.dimensions.into();
        Self {
            dimensions,
            icon: metadata.icon.into(),
            ports: metadata
                .ports
                .into_iter()
                .map(|port| {
                    let mut port: Port = port.into();
                    port.location.x = port.location.x.min(dimensions.width());
                    port.location.y = port.location.y.min(dimensions.height());
                    port
                })
                .collect(),
        }
    }
}

/// The version 1 JSON representation of a GUI port.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PortReaderV1 {
    /// Validated identifier of the port, represented by a plain string.
    #[serde(deserialize_with = "deserialize_port_key")]
    id: PortKey,
    /// The role of the connection, represented by a plain string.
    #[serde(rename = "type", deserialize_with = "deserialize_unit_variant")]
    port_type: PortTypeReaderV1,
    /// Nonnegative icon-local drawing coordinates.
    #[serde(deserialize_with = "deserialize_object")]
    location: PortLocationReaderV1,
}

impl From<PortReaderV1> for Port {
    fn from(port: PortReaderV1) -> Self {
        Self {
            id: port.id,
            port_type: port.port_type.into(),
            location: port.location.into(),
        }
    }
}

/// The version 1 names for connection roles.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum PortTypeReaderV1 {
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

impl From<PortTypeReaderV1> for PortType {
    fn from(port_type: PortTypeReaderV1) -> Self {
        match port_type {
            PortTypeReaderV1::Input => Self::Input,
            PortTypeReaderV1::Output => Self::Output,
            PortTypeReaderV1::Power => Self::Power,
            PortTypeReaderV1::BidirectionalSignal => Self::BidirectionalSignal,
            PortTypeReaderV1::System => Self::System,
            PortTypeReaderV1::PowerMulti => Self::PowerMulti,
            PortTypeReaderV1::InputMulti => Self::InputMulti,
        }
    }
}

/// The version 1 JSON representation of an icon-local location.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PortLocationReaderV1 {
    /// Horizontal drawing coordinate.
    x: u16,
    /// Vertical drawing coordinate.
    y: u16,
}

impl From<PortLocationReaderV1> for PortLocation {
    fn from(location: PortLocationReaderV1) -> Self {
        Self {
            x: location.x,
            y: location.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GuiMetadataReaderV1;
    use crate::common::deserialize::RootObject;
    use crate::gui_metadata::data::{GuiMetadata, PortLocation};
    use serde_json::{Value, json};

    /// Reads GUI metadata through its object-only envelope.
    fn read(value: &Value) -> Result<GuiMetadata, serde_json::Error> {
        let reader: RootObject<GuiMetadataReaderV1> =
            serde_json::from_str(&serde_json::to_string(value)?)?;
        Ok(reader.document.into())
    }

    /// Creates GUI metadata with explicit dimensions and an unresolved icon path.
    fn metadata_json() -> Value {
        json!({
            "dimensions": {"width": 100, "height": 50},
            "icon": "../icons/missing.svg",
            "ports": [],
        })
    }

    /// Creates a signal output at the right-edge midpoint.
    fn port_json() -> Value {
        json!({
            "id": "temperature",
            "type": "output",
            "location": {"x": 100, "y": 25},
        })
    }

    #[test]
    fn ports_require_an_array_of_objects() {
        for invalid in [
            Value::Null,
            json!(false),
            json!("ports"),
            json!({}),
            port_json(),
            json!([null]),
            json!([["temperature", "output", {"x": 100, "y": 25}]]),
        ] {
            let mut value = metadata_json();
            value["ports"] = invalid;
            assert!(read(&value).is_err(), "{value}");
        }
    }

    #[test]
    fn port_fields_are_required_and_typed() {
        let mut invalid_ports = Vec::new();
        for field in ["id", "type", "location"] {
            let mut port = port_json();
            port.as_object_mut().unwrap().remove(field);
            invalid_ports.push(port);
        }
        for field in ["x", "y"] {
            let mut port = port_json();
            port["location"].as_object_mut().unwrap().remove(field);
            invalid_ports.push(port);
        }
        let mut port = port_json();
        port["extra"] = json!(1);
        invalid_ports.push(port);
        let mut port = port_json();
        port["location"]["extra"] = json!(1);
        invalid_ports.push(port);
        for invalid in [Value::Null, json!(false), json!(100), json!([]), json!({})] {
            let mut port = port_json();
            port["id"] = invalid;
            invalid_ports.push(port);
        }
        for invalid in [
            Value::Null,
            json!(false),
            json!(100),
            json!("location"),
            json!([100, 25]),
        ] {
            let mut port = port_json();
            port["location"] = invalid;
            invalid_ports.push(port);
        }
        for port in invalid_ports {
            let mut value = metadata_json();
            value["ports"] = json!([port]);
            let error = read(&value).unwrap_err();
            assert!(error.is_data(), "{value}: {error}");
            assert!(error.line() > 0);
            assert!(error.column() > 0);
        }
    }

    #[test]
    fn port_types_require_known_plain_strings() {
        for invalid in [
            json!("Output"),
            json!("unknown"),
            json!(""),
            json!({"output": null}),
            json!(["output"]),
            json!({}),
            json!(100),
            json!(false),
            Value::Null,
        ] {
            let mut port = port_json();
            port["type"] = invalid;
            let mut value = metadata_json();
            value["ports"] = json!([port]);
            assert!(read(&value).is_err(), "{value}");
        }
    }

    #[test]
    fn port_ids_are_validated_port_keys() {
        for id in ["a", "temperature", "temperature_out1", "sensor2"] {
            let mut port = port_json();
            port["id"] = json!(id);
            let mut value = metadata_json();
            value["ports"] = json!([port]);
            let metadata = read(&value).unwrap();
            assert_eq!(metadata.ports[0].id.as_str(), id);
        }
        for (id, diagnostic) in [
            ("", "datastore_key_empty"),
            ("true", "datastore_key_reserved"),
            ("false", "datastore_key_reserved"),
            ("a_temperature", "datastore_key_invalid_prefix"),
            ("z_temperature", "datastore_key_invalid_prefix"),
            ("Temperature", "datastore_key_invalid_character"),
            ("1temperature", "datastore_key_invalid_character"),
            ("temperature-out", "datastore_key_invalid_character"),
            ("temperature out", "datastore_key_invalid_character"),
            ("temperature \u{3b1}", "datastore_key_invalid_character"),
        ] {
            let mut port = port_json();
            port["id"] = json!(id);
            let mut value = metadata_json();
            value["ports"] = json!([port]);
            let error = read(&value).unwrap_err();
            assert!(error.is_data(), "{id}: {error}");
            assert!(error.to_string().contains(diagnostic), "{id}: {error}");
            assert!(error.line() > 0);
            assert!(error.column() > 0);
        }
    }

    #[test]
    fn port_locations_require_unsigned_16_bit_integers() {
        for field in ["x", "y"] {
            for invalid in [
                json!(-1),
                json!(65536),
                json!(1.5),
                json!(25.0),
                json!("25"),
                Value::Null,
                json!(true),
                json!([]),
                json!({}),
            ] {
                let mut port = port_json();
                port["location"][field] = invalid;
                let mut value = metadata_json();
                value["ports"] = json!([port]);
                assert!(read(&value).is_err(), "{value}");
            }
        }
        for coordinate in ["1e1", "1.0"] {
            let text = format!(
                r#"{{"dimensions":{{"width":100,"height":50}},"icon":"","ports":[{{"id":"p","type":"input","location":{{"x":{coordinate},"y":0}}}}]}}"#,
            );
            assert!(serde_json::from_str::<RootObject<GuiMetadataReaderV1>>(&text).is_err());
        }
    }

    #[test]
    fn port_locations_are_capped_to_icon_dimensions() {
        for (width, height) in [(100, 50), (0, 0), (0, u16::MAX), (u16::MAX, 0)] {
            for (x, y) in [
                (101, 25),
                (100, 51),
                (u16::MAX, 0),
                (0, u16::MAX),
                (u16::MAX, u16::MAX),
            ] {
                let mut port = port_json();
                port["location"] = json!({"x": x, "y": y});
                let mut value = metadata_json();
                value["dimensions"] = json!({"width": width, "height": height});
                value["ports"] = json!([port_json(), port]);
                let metadata = read(&value).unwrap();
                assert_eq!(
                    metadata.ports[0].location,
                    PortLocation {
                        x: 100.min(width),
                        y: 25.min(height),
                    },
                );
                assert_eq!(
                    metadata.ports[1].location,
                    PortLocation {
                        x: x.min(width),
                        y: y.min(height),
                    },
                );
                assert_eq!(metadata.ports[1].id.as_str(), "temperature");
            }
        }
    }

    #[test]
    fn duplicate_port_and_location_fields_are_rejected() {
        for port in [
            r#"{"id":"p","id":"q","type":"output","location":{"x":0,"y":0}}"#,
            r#"{"id":"p","type":"output","type":"input","location":{"x":0,"y":0}}"#,
            r#"{"id":"p","type":"output","location":{"x":0,"y":0},"location":{"x":1,"y":1}}"#,
            r#"{"id":"p","type":"output","location":{"x":0,"x":1,"y":0}}"#,
            r#"{"id":"p","type":"output","location":{"x":0,"y":0,"y":1}}"#,
        ] {
            let text = format!(
                r#"{{"dimensions":{{"width":100,"height":50}},"icon":"","ports":[{port}]}}"#,
            );
            assert!(serde_json::from_str::<RootObject<GuiMetadataReaderV1>>(&text).is_err());
        }
    }

    #[test]
    fn dimensions_and_icon_paths_are_preserved() {
        for path in [
            "",
            "../icons/missing \u{3b1}.svg",
            " leading and trailing spaces ",
            r"C:\icons\missing.png",
        ] {
            let mut value = metadata_json();
            value["icon"] = json!(path);
            let metadata = read(&value).unwrap();
            assert_eq!(metadata.dimensions.width(), 100);
            assert_eq!(metadata.dimensions.height(), 50);
            assert_eq!(metadata.icon.as_str(), path);
        }
    }

    #[test]
    fn icon_rejects_non_string_values() {
        for invalid in [Value::Null, json!(123), json!(false), json!([]), json!({})] {
            let mut value = metadata_json();
            value["icon"] = invalid;
            let error = read(&value).unwrap_err();
            assert!(error.is_data(), "{error}");
            assert!(error.line() > 0);
            assert!(error.column() > 0);
        }
    }

    #[test]
    fn dimensions_require_unsigned_16_bit_integers() {
        for size in [0, u16::MAX] {
            let mut value = metadata_json();
            value["dimensions"] = json!({"width": size, "height": size});
            let metadata = read(&value).unwrap();
            assert_eq!(metadata.dimensions.width(), size);
            assert_eq!(metadata.dimensions.height(), size);
        }
        for field in ["width", "height"] {
            for invalid in [
                json!(-1),
                json!(65536),
                json!(1.5),
                json!(100.0),
                json!("100"),
                Value::Null,
                json!(true),
                json!([]),
                json!({}),
            ] {
                let mut value = metadata_json();
                value["dimensions"][field] = invalid;
                assert!(read(&value).is_err(), "{value}");
            }
        }
    }

    #[test]
    fn fields_are_required_and_unknown_fields_are_rejected() {
        for field in ["dimensions", "icon", "ports"] {
            let mut value = metadata_json();
            value.as_object_mut().unwrap().remove(field);
            assert!(read(&value).is_err(), "{value}");
        }
        for field in ["width", "height"] {
            let mut value = metadata_json();
            value["dimensions"].as_object_mut().unwrap().remove(field);
            assert!(read(&value).is_err(), "{value}");
        }
        let mut value = metadata_json();
        value["extra"] = json!(1);
        assert!(read(&value).is_err());
        let mut value = metadata_json();
        value["dimensions"]["extra"] = json!(1);
        assert!(read(&value).is_err());
    }

    #[test]
    fn structs_require_json_objects() {
        for invalid in [
            json!([{"width": 100, "height": 50}, "icon.svg", []]),
            Value::Null,
            json!(false),
            json!(100),
            json!("metadata"),
        ] {
            assert!(read(&invalid).is_err(), "{invalid}");
        }
        for invalid in [
            json!([100, 50]),
            Value::Null,
            json!(false),
            json!(100),
            json!("dimensions"),
        ] {
            let mut value = metadata_json();
            value["dimensions"] = invalid;
            assert!(read(&value).is_err(), "{value}");
        }
    }

    #[test]
    fn duplicate_fields_are_rejected() {
        for json in [
            r#"{"dimensions":{"width":100,"height":50},"icon":"a.svg","icon":"b.svg","ports":[]}"#,
            r#"{"dimensions":{"width":100,"height":50},"dimensions":{"width":1,"height":2},"icon":"","ports":[]}"#,
            r#"{"dimensions":{"width":100,"width":1,"height":50},"icon":"","ports":[]}"#,
            r#"{"dimensions":{"width":100,"height":50,"height":1},"icon":"","ports":[]}"#,
            r#"{"dimensions":{"width":100,"height":50},"icon":"","ports":[],"ports":[]}"#,
        ] {
            assert!(
                serde_json::from_str::<RootObject<GuiMetadataReaderV1>>(json).is_err(),
                "{json}"
            );
        }
    }
}
