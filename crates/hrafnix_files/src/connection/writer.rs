use crate::common::serialize::{serialize_component_key, serialize_port_key};
use crate::connection::data::{Connection, ConnectionLine};
use crate::graphics::writer::PositionWriter;
use hrafnix_keys::{ComponentKey, PortKey};
use serde::Serialize;

/// The current JSON representation of a connection used when writing.
#[derive(Debug, Serialize)]
pub(crate) struct ConnectionWriter {
    /// Validated source component identifier, represented by a plain string.
    #[serde(serialize_with = "serialize_component_key")]
    from_component: ComponentKey,
    /// Validated source port identifier, represented by a plain string.
    #[serde(serialize_with = "serialize_port_key")]
    from_port: PortKey,
    /// Validated destination component identifier, represented by a plain string.
    #[serde(serialize_with = "serialize_component_key")]
    to_component: ComponentKey,
    /// Validated destination port identifier, represented by a plain string.
    #[serde(serialize_with = "serialize_port_key")]
    to_port: PortKey,
    /// Drawing path between the connected ports.
    line: ConnectionLineWriter,
}

impl From<&Connection> for ConnectionWriter {
    fn from(connection: &Connection) -> Self {
        Self {
            from_component: connection.from_component.clone(),
            from_port: connection.from_port.clone(),
            to_component: connection.to_component.clone(),
            to_port: connection.to_port.clone(),
            line: ConnectionLineWriter::from(&connection.line),
        }
    }
}

/// The current JSON representation of a connection's drawing path.
#[derive(Debug, Serialize)]
struct ConnectionLineWriter {
    /// Ordered midpoints in drawing coordinates.
    midpoints: Vec<PositionWriter>,
}

impl From<&ConnectionLine> for ConnectionLineWriter {
    fn from(line: &ConnectionLine) -> Self {
        Self {
            midpoints: line.midpoints.iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConnectionWriter;
    use crate::common::deserialize::RootObject;
    use crate::connection::data::{Connection, ConnectionLine};
    use crate::connection::reader::v1::ConnectionReaderV1;
    use crate::graphics::data::Position;
    use hrafnix_keys::{component_key, port_key};
    use serde_json::json;

    #[test]
    fn connections_match_the_reader_schema_without_style() {
        for midpoints in [
            vec![],
            vec![Position::new(210, 25)],
            vec![
                Position::new(i32::MIN, i32::MAX),
                Position::new(-10, 0),
                Position::new(i32::MAX, i32::MIN),
            ],
        ] {
            let connection = Connection {
                from_component: component_key!("simulation_clock").into(),
                from_port: port_key!("time").into(),
                to_component: component_key!("heater").into(),
                to_port: port_key!("temperature_input").into(),
                line: ConnectionLine { midpoints },
            };
            let original = connection.clone();
            let written = serde_json::to_value(ConnectionWriter::from(&connection)).unwrap();
            assert_eq!(
                written,
                json!({
                    "from_component": "simulation_clock",
                    "from_port": "time",
                    "to_component": "heater",
                    "to_port": "temperature_input",
                    "line": {
                        "midpoints": connection.line.midpoints.iter()
                            .map(|point| json!({"x": point.x(), "y": point.y()}))
                            .collect::<Vec<_>>(),
                    },
                }),
            );
            let reader: RootObject<ConnectionReaderV1> =
                serde_json::from_str(&written.to_string()).unwrap();
            assert_eq!(Connection::from(reader.document), connection);
            assert_eq!(connection, original);
        }
    }
}
