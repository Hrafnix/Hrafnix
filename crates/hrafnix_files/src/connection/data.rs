use crate::graphics::data::Position;
use hrafnix_keys::{ComponentKey, PortKey};
use hrafnix_shareable_string::SharedStringStore;

/// A directed connection between two component ports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connection {
    /// Validated identifier of the source component.
    from_component: ComponentKey,
    /// Validated identifier of the source port.
    from_port: PortKey,
    /// Validated identifier of the destination component.
    to_component: ComponentKey,
    /// Validated identifier of the destination port.
    to_port: PortKey,
    /// Drawing path between the connected ports.
    line: ConnectionLine,
}

impl Connection {
    /// Creates a new directed connection between two component ports.
    #[must_use]
    pub(crate) const fn new(
        from_component: ComponentKey,
        from_port: PortKey,
        to_component: ComponentKey,
        to_port: PortKey,
        line: ConnectionLine,
    ) -> Self {
        Self {
            from_component,
            from_port,
            to_component,
            to_port,
            line,
        }
    }

    /// Returns the validated identifier of the source component.
    #[must_use]
    pub const fn from_component(&self) -> &ComponentKey {
        &self.from_component
    }

    /// Returns the validated identifier of the source port.
    #[must_use]
    pub const fn from_port(&self) -> &PortKey {
        &self.from_port
    }

    /// Returns the validated identifier of the destination component.
    #[must_use]
    pub const fn to_component(&self) -> &ComponentKey {
        &self.to_component
    }

    /// Returns the validated identifier of the destination port.
    #[must_use]
    pub const fn to_port(&self) -> &PortKey {
        &self.to_port
    }

    /// Returns the drawing path between the connected ports.
    #[must_use]
    pub const fn line(&self) -> &ConnectionLine {
        &self.line
    }

    /// Interns this value's strings in `store` in place, preserving its contents.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.from_component.launder(store);
        self.from_port.launder(store);
        self.to_component.launder(store);
        self.to_port.launder(store);
    }
}

/// Intermediate points along a connection's drawing path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionLine {
    /// Ordered midpoints in signed 32-bit drawing coordinates.
    midpoints: Vec<Position>,
}

impl ConnectionLine {
    /// Creates a new connection line with the given ordered midpoints.
    #[must_use]
    pub(crate) const fn new(midpoints: Vec<Position>) -> Self {
        Self { midpoints }
    }

    /// Returns the ordered midpoints along the connection's drawing path.
    #[must_use]
    pub fn midpoints(&self) -> &[Position] {
        &self.midpoints
    }
}
