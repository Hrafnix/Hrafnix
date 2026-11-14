use crate::graphics::data::Position;
use hrafnix_keys::{ComponentKey, PortKey};

/// A directed connection between two component ports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connection {
    /// Validated identifier of the source component.
    pub from_component: ComponentKey,
    /// Validated identifier of the source port.
    pub from_port: PortKey,
    /// Validated identifier of the destination component.
    pub to_component: ComponentKey,
    /// Validated identifier of the destination port.
    pub to_port: PortKey,
    /// Drawing path between the connected ports.
    pub line: ConnectionLine,
}

/// Intermediate points along a connection's drawing path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionLine {
    /// Ordered midpoints in signed 32-bit drawing coordinates.
    pub midpoints: Vec<Position>,
}
