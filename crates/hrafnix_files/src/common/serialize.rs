use hrafnix_keys::{ComponentKey, PortKey};
use hrafnix_shareable_string::ShareableString;
use serde::Serializer;

/// Serializes a shared string as a borrowed string slice without copying it.
pub(crate) fn serialize_shareable_string<S: Serializer>(
    value: &ShareableString,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(value.as_str())
}

/// Serializes a component key using its underlying shared string.
pub(crate) fn serialize_component_key<S: Serializer>(
    key: &ComponentKey,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serialize_shareable_string(key.as_shareable_string(), serializer)
}

/// Serializes a port key using its underlying shared string.
pub(crate) fn serialize_port_key<S: Serializer>(
    key: &PortKey,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serialize_shareable_string(key.as_shareable_string(), serializer)
}
