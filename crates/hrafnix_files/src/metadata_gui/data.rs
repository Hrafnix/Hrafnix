use crate::graphics::data::Dimension;
use camino::Utf8PathBuf;
use hrafnix_common::port::PortType;
use hrafnix_keys::PortKey;

/// Drawing dimensions, icon, and port locations associated with a component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiMetadata {
    /// Dimensions in drawing coordinates.
    pub dimensions: Dimension,
    /// Icon path, stored unchanged rather than opened or resolved during file I/O.
    pub icon: Utf8PathBuf,
    /// Connections placed within the icon's drawing dimensions.
    pub ports: Vec<Port>,
}

/// A connection placed on a component's icon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port {
    /// Validated identifier of the port.
    pub id: PortKey,
    /// The role of the connection.
    pub port_type: PortType,
    /// Location within the icon's drawing dimensions.
    pub location: PortLocation,
}

/// A nonnegative location in icon-local drawing coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortLocation {
    /// Horizontal coordinate, capped to the icon width when reading and writing.
    pub x: u16,
    /// Vertical coordinate, capped to the icon height when reading and writing.
    pub y: u16,
}
