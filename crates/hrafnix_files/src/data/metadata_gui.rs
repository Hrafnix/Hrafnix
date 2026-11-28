use crate::data::graphics::{Dimension, Rotation};
use camino::{Utf8Path, Utf8PathBuf};
use hrafnix_common::port::PortType;
use hrafnix_keys::PortKey;
use hrafnix_shareable_string::SharedStringStore;

/// Drawing dimensions, rotation, icon, and port locations associated with a component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiMetadata {
    /// Dimensions in drawing coordinates.
    dimensions: Dimension,
    /// Clockwise rotation of the icon and ports about the center of their bounds.
    rotation: Rotation,
    /// Icon path, stored unchanged rather than opened or resolved during file I/O.
    icon: Utf8PathBuf,
    /// Connections placed within the icon's drawing dimensions.
    ports: Vec<Port>,
}

impl GuiMetadata {
    /// Creates new GUI metadata with the given dimensions, rotation, icon path, and ports.
    #[must_use]
    pub(crate) const fn new(
        dimensions: Dimension,
        rotation: Rotation,
        icon: Utf8PathBuf,
        ports: Vec<Port>,
    ) -> Self {
        Self {
            dimensions,
            rotation,
            icon,
            ports,
        }
    }

    /// Returns the unrotated dimensions in drawing coordinates.
    #[must_use]
    pub const fn dimensions(&self) -> Dimension {
        self.dimensions
    }

    /// Returns the clockwise rotation about the center of the unrotated icon bounds.
    #[must_use]
    pub const fn rotation(&self) -> Rotation {
        self.rotation
    }

    /// Returns the icon path.
    #[must_use]
    pub fn icon(&self) -> &Utf8Path {
        self.icon.as_path()
    }

    /// Returns the ports placed within the icon's drawing dimensions.
    #[must_use]
    pub fn ports(&self) -> &[Port] {
        &self.ports
    }

    /// Interns this value's strings in `store` in place, preserving its contents.
    pub fn launder(&mut self, store: &SharedStringStore) {
        for port in &mut self.ports {
            port.launder(store);
        }
    }
}

/// A connection placed on a component's icon.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::struct_field_names)]
pub struct Port {
    /// Validated identifier of the port.
    id: PortKey,
    /// The role of the connection.
    port_type: PortType,
    /// Location within the icon's drawing dimensions.
    location: PortLocation,
}

impl Port {
    /// Creates a new port with the given identifier, type, and location.
    #[must_use]
    pub(crate) const fn new(id: PortKey, port_type: PortType, location: PortLocation) -> Self {
        Self {
            id,
            port_type,
            location,
        }
    }

    /// Returns the validated identifier of the port.
    #[must_use]
    pub const fn id(&self) -> &PortKey {
        &self.id
    }

    /// Returns the role of the connection.
    #[must_use]
    pub const fn port_type(&self) -> PortType {
        self.port_type
    }

    /// Returns the location within the icon's drawing dimensions.
    #[must_use]
    pub const fn location(&self) -> PortLocation {
        self.location
    }

    /// Interns this value's strings in `store` in place, preserving its contents.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.id.launder(store);
    }
}

/// A nonnegative location in icon-local drawing coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortLocation {
    /// Horizontal coordinate, capped to the icon width when reading and writing.
    x: u16,
    /// Vertical coordinate, capped to the icon height when reading and writing.
    y: u16,
}

impl PortLocation {
    /// Creates a new port location with the given coordinates.
    #[must_use]
    pub(crate) const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// Returns the horizontal coordinate.
    #[must_use]
    pub const fn x(&self) -> u16 {
        self.x
    }

    /// Returns the vertical coordinate.
    #[must_use]
    pub const fn y(&self) -> u16 {
        self.y
    }
}
