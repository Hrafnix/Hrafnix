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
