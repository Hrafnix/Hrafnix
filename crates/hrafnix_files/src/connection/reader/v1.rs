use crate::common::deserialize::{deserialize_object, deserialize_objects};
use crate::connection::data::{Connection, ConnectionLine};
use crate::graphics::reader::v1::PositionReaderV1;
use hrafnix_keys::{ComponentKey, PortKey};
use hrafnix_message::message::Message;
use serde::Deserialize;

/// The version 1 JSON representation of a connection.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ConnectionReaderV1 {
    /// Plain-string source component identifier, validated during conversion.
    from_component: String,
    /// Plain-string source port identifier, validated during conversion.
    from_port: String,
    /// Plain-string destination component identifier, validated during conversion.
    to_component: String,
    /// Plain-string destination port identifier, validated during conversion.
    to_port: String,
    /// Drawing path between the connected ports.
    #[serde(deserialize_with = "deserialize_object")]
    line: ConnectionLineReaderV1,
}

impl TryFrom<ConnectionReaderV1> for Connection {
    type Error = Message;

    fn try_from(connection: ConnectionReaderV1) -> Result<Self, Self::Error> {
        Ok(Self::new(
            ComponentKey::new(connection.from_component.into())?,
            PortKey::new(connection.from_port.into())?,
            ComponentKey::new(connection.to_component.into())?,
            PortKey::new(connection.to_port.into())?,
            connection.line.into(),
        ))
    }
}

/// The version 1 JSON representation of a connection's drawing path.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConnectionLineReaderV1 {
    /// Ordered midpoints in drawing coordinates.
    #[serde(deserialize_with = "deserialize_objects")]
    midpoints: Vec<PositionReaderV1>,
}

impl From<ConnectionLineReaderV1> for ConnectionLine {
    fn from(line: ConnectionLineReaderV1) -> Self {
        Self::new(line.midpoints.into_iter().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::ConnectionReaderV1;
    use crate::common::deserialize::RootObject;
    use crate::common::test_support::{ReadError, read_value};
    use crate::connection::data::Connection;
    use crate::graphics::data::Position;
    use serde_json::{Value, json};

    /// Reads a connection through its object-only envelope.
    fn read(value: &Value) -> Result<Connection, ReadError> {
        read_value::<ConnectionReaderV1, _>(value)
    }

    /// Creates a connection from the simulation clock to the heater.
    fn connection_json() -> Value {
        json!({
            "from_component": "simulation_clock",
            "from_port": "time",
            "to_component": "heater",
            "to_port": "temperature_input",
            "line": {"midpoints": [{"x": 210, "y": 25}]},
        })
    }

    #[test]
    fn endpoints_and_midpoint_order_are_preserved() {
        let mut value = connection_json();
        value["line"]["midpoints"] = json!([
            {"x": 210, "y": 25},
            {"x": i32::MIN, "y": i32::MAX},
            {"x": -10, "y": 0},
            {"x": 210, "y": 25},
        ]);
        let connection = read(&value).unwrap();
        assert_eq!(connection.from_component().as_str(), "simulation_clock");
        assert_eq!(connection.from_port().as_str(), "time");
        assert_eq!(connection.to_component().as_str(), "heater");
        assert_eq!(connection.to_port().as_str(), "temperature_input");
        assert_eq!(
            connection.line().midpoints(),
            &[
                Position::new(210, 25),
                Position::new(i32::MIN, i32::MAX),
                Position::new(-10, 0),
                Position::new(210, 25),
            ],
        );
        value["line"]["midpoints"] = json!([]);
        assert_eq!(read(&value).unwrap().line().midpoints(), &[]);
    }

    #[test]
    fn fields_are_required_and_unknown_fields_including_style_are_rejected() {
        for pointer in ["", "/line", "/line/midpoints/0"] {
            let value = connection_json();
            for field in value.pointer(pointer).unwrap().as_object().unwrap().keys() {
                let mut invalid = value.clone();
                invalid
                    .pointer_mut(pointer)
                    .unwrap()
                    .as_object_mut()
                    .unwrap()
                    .remove(field);
                assert!(read(&invalid).is_err(), "{invalid}");
            }
            for field in ["unknown", "style"] {
                let mut invalid = value.clone();
                invalid.pointer_mut(pointer).unwrap()[field] = json!("solid");
                assert!(read(&invalid).is_err(), "{invalid}");
            }
        }
    }

    #[test]
    fn structs_require_objects_and_midpoints_require_an_array_of_objects() {
        for pointer in ["", "/line", "/line/midpoints/0"] {
            for invalid in [
                Value::Null,
                json!(true),
                json!(1),
                json!("text"),
                json!([]),
                json!([210, 25]),
                json!([
                    "simulation_clock",
                    "time",
                    "heater",
                    "temperature_input",
                    []
                ]),
            ] {
                let mut value = connection_json();
                *value.pointer_mut(pointer).unwrap() = invalid;
                assert!(read(&value).is_err(), "{value}");
            }
        }
        for invalid in [
            Value::Null,
            json!(true),
            json!(1),
            json!("text"),
            json!({}),
            json!({"x": 210, "y": 25}),
            json!([null]),
            json!([[210, 25]]),
        ] {
            let mut value = connection_json();
            value["line"]["midpoints"] = invalid;
            assert!(read(&value).is_err(), "{value}");
        }
    }

    #[test]
    fn endpoints_require_validated_plain_string_keys() {
        for field in ["from_component", "from_port", "to_component", "to_port"] {
            for key in ["a", "sensor2", "temperature_out1"] {
                let mut value = connection_json();
                value[field] = json!(key);
                read(&value).unwrap();
            }
            for (key, diagnostic) in [
                ("", "datastore_key_empty"),
                ("true", "datastore_key_reserved"),
                ("false", "datastore_key_reserved"),
                ("a_sensor", "datastore_key_invalid_prefix"),
                ("z_sensor", "datastore_key_invalid_prefix"),
                ("Sensor", "datastore_key_invalid_character"),
                ("1sensor", "datastore_key_invalid_character"),
                ("sensor-out", "datastore_key_invalid_character"),
                ("sensor out", "datastore_key_invalid_character"),
                ("sensor\u{3b1}", "datastore_key_invalid_character"),
            ] {
                let mut value = connection_json();
                value[field] = json!(key);
                let error = read(&value).unwrap_err();
                assert_eq!(error.message_key(), diagnostic, "{value}");
            }
            for invalid in [Value::Null, json!(true), json!(1), json!([]), json!({})] {
                let mut value = connection_json();
                value[field] = invalid;
                assert!(read(&value).is_err(), "{value}");
            }
        }
    }

    #[test]
    fn midpoints_require_signed_32_bit_integers() {
        for field in ["x", "y"] {
            for invalid in [
                json!(-2_147_483_649_i64),
                json!(2_147_483_648_i64),
                json!(1.5),
                json!(25.0),
                json!("25"),
                Value::Null,
                json!(true),
                json!([]),
                json!({}),
            ] {
                let mut value = connection_json();
                value["line"]["midpoints"][0][field] = invalid;
                assert!(read(&value).is_err(), "{value}");
            }
            for coordinate in ["1e1", "1.0"] {
                let mut value = connection_json();
                value["line"]["midpoints"][0][field] = json!("coordinate");
                let text = serde_json::to_string(&value)
                    .unwrap()
                    .replace("\"coordinate\"", coordinate);
                assert!(serde_json::from_str::<RootObject<ConnectionReaderV1>>(&text).is_err());
            }
        }
    }

    #[test]
    fn duplicate_fields_are_rejected_at_every_level() {
        for pointer in ["", "/line", "/line/midpoints/0"] {
            let value = connection_json();
            for (field, entry) in value.pointer(pointer).unwrap().as_object().unwrap() {
                let mut invalid = value.clone();
                invalid.pointer_mut(pointer).unwrap()[field] = json!("duplicate");
                let text = serde_json::to_string(&invalid).unwrap().replace(
                    &format!("\"{field}\":\"duplicate\""),
                    &format!("\"{field}\":{entry},\"{field}\":{entry}"),
                );
                let error =
                    serde_json::from_str::<RootObject<ConnectionReaderV1>>(&text).unwrap_err();
                assert!(
                    error.to_string().contains("duplicate field"),
                    "{text}: {error}"
                );
            }
        }
    }
}
