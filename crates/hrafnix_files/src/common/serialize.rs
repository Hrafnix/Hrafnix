use hrafnix_keys::{ComponentKey, GlobalKey, ParameterKey, PortKey, StoreKey, VariableKey};
use hrafnix_shareable_string::ShareableString;
use serde::{Serialize, Serializer};

/// Serializes a shared string as a borrowed string slice without copying it.
pub(crate) fn serialize_shareable_string<S: Serializer>(
    value: &ShareableString,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(value.as_str())
}

/// Serializes shared strings as an array of plain strings.
pub(crate) fn serialize_shareable_strings<S: Serializer>(
    values: &[ShareableString],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    values
        .iter()
        .map(ShareableString::as_str)
        .collect::<Vec<_>>()
        .serialize(serializer)
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

/// Serializes a datastore key using its underlying shared string.
pub(crate) fn serialize_store_key<S: Serializer>(
    key: &StoreKey,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serialize_shareable_string(key.as_shareable_string(), serializer)
}

/// Serializes a global key using its underlying shared string.
pub(crate) fn serialize_global_key<S: Serializer>(
    key: &GlobalKey,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serialize_shareable_string(key.as_shareable_string(), serializer)
}

/// Serializes a parameter key using its underlying shared string.
pub(crate) fn serialize_parameter_key<S: Serializer>(
    key: &ParameterKey,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serialize_shareable_string(key.as_shareable_string(), serializer)
}

/// Serializes a variable key using its underlying shared string.
pub(crate) fn serialize_variable_key<S: Serializer>(
    key: &VariableKey,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serialize_shareable_string(key.as_shareable_string(), serializer)
}
