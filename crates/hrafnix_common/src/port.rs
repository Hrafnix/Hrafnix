/// The role of a signal or physical connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortType {
    /// Receives a signal.
    Input,
    /// Emits a signal.
    Output,
    /// Exchanges physical power.
    Power,
    /// Exchanges signals in both directions.
    BidirectionalSignal,
    /// Exposes a connection at a subsystem boundary.
    System,
    /// Contains multiple physical subports.
    PowerMulti,
    /// Contains multiple signal-input subports.
    InputMulti,
}

/// The kind of component port.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortKind {
    /// A port that accepts a signal input.
    SignalInput,
    /// A port that produces a signal output.
    SignalOutput,
    /// A power port for hydraulic pressure and fluid flow.
    Hydraulic,
    /// A power port for pneumatic pressure, energy flow, and mass flow.
    Pneumatic,
    /// A power port for translational force and velocity.
    TranslationalMechanical,
    /// A power port for rotational torque and angular velocity.
    RotationalMechanical,
    /// A power port for electric voltage and current.
    Electric,
    /// A power port for planar mechanical force, velocity, and torque.
    PlanarMechanical,
}

impl PortKind {
    /// Returns the connection role used by saved component interfaces.
    #[must_use]
    pub const fn port_type(self) -> PortType {
        match self {
            Self::SignalInput => PortType::Input,
            Self::SignalOutput => PortType::Output,
            Self::Hydraulic
            | Self::Pneumatic
            | Self::TranslationalMechanical
            | Self::RotationalMechanical
            | Self::Electric
            | Self::PlanarMechanical => PortType::Power,
        }
    }

    /// Returns the port kind with the given stable identifier.
    #[must_use]
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "signal_input" => Some(Self::SignalInput),
            "signal_output" => Some(Self::SignalOutput),
            "hydraulic" => Some(Self::Hydraulic),
            "pneumatic" => Some(Self::Pneumatic),
            "translational_mechanical" => Some(Self::TranslationalMechanical),
            "rotational_mechanical" => Some(Self::RotationalMechanical),
            "electric" => Some(Self::Electric),
            "planar_mechanical" => Some(Self::PlanarMechanical),
            _ => None,
        }
    }

    /// Returns the stable string representation of this port kind.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignalInput => "signal_input",
            Self::SignalOutput => "signal_output",
            Self::Hydraulic => "hydraulic",
            Self::Pneumatic => "pneumatic",
            Self::TranslationalMechanical => "translational_mechanical",
            Self::RotationalMechanical => "rotational_mechanical",
            Self::Electric => "electric",
            Self::PlanarMechanical => "planar_mechanical",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PortKind, PortType};

    #[test]
    fn kinds_preserve_signal_and_physical_connection_roles() {
        assert_eq!(PortKind::SignalInput.port_type(), PortType::Input);
        assert_eq!(PortKind::SignalOutput.port_type(), PortType::Output);
        for kind in [
            PortKind::Hydraulic,
            PortKind::Pneumatic,
            PortKind::TranslationalMechanical,
            PortKind::RotationalMechanical,
            PortKind::Electric,
            PortKind::PlanarMechanical,
        ] {
            assert_eq!(kind.port_type(), PortType::Power);
        }
    }

    #[test]
    fn identifiers_round_trip() {
        let port_kinds = [
            PortKind::SignalInput,
            PortKind::SignalOutput,
            PortKind::Hydraulic,
            PortKind::Pneumatic,
            PortKind::TranslationalMechanical,
            PortKind::RotationalMechanical,
            PortKind::Electric,
            PortKind::PlanarMechanical,
        ];

        for port_kind in port_kinds {
            assert_eq!(PortKind::from_id(port_kind.as_str()), Some(port_kind));
        }
    }

    #[test]
    fn unknown_identifier_returns_none() {
        assert_eq!(PortKind::from_id("unknown"), None);
    }
}
