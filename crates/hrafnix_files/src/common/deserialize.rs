use hrafnix_keys::PortKey;
use serde::de::value::{MapAccessDeserializer, StrDeserializer};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::{BTreeMap, btree_map::Entry};
use std::fmt;
use std::marker::PhantomData;

/// A document that must be written as a JSON object rather than a positional array.
#[derive(Debug)]
pub(crate) struct RootObject<T> {
    /// The deserialized document.
    pub(crate) document: T,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for RootObject<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserialize_object(deserializer).map(|document| Self { document })
    }
}

/// Deserializes an object as `T`, rejecting positional arrays for derived structs.
pub(crate) fn deserialize_object<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<T, D::Error> {
    deserializer.deserialize_map(ObjectVisitor {
        document: PhantomData,
    })
}

/// Deserializes an array of objects, rejecting positional arrays for its elements.
pub(crate) fn deserialize_objects<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<T>, D::Error> {
    Vec::<RootObject<T>>::deserialize(deserializer)
        .map(|objects| objects.into_iter().map(|object| object.document).collect())
}

/// Deserializes an object without silently overwriting duplicate entries.
pub(crate) fn deserialize_entries<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<BTreeMap<String, T>, D::Error> {
    deserializer.deserialize_map(EntriesVisitor { value: PhantomData })
}

/// Deserializes a unit-only enum from a plain string, rejecting the object form
/// (such as `{"Variant": null}`) that Serde otherwise accepts.
pub(crate) fn deserialize_unit_variant<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<T, D::Error> {
    deserializer.deserialize_str(UnitVariantVisitor {
        variant: PhantomData,
    })
}

/// Deserializes a plain string through the shared port-key validator.
pub(crate) fn deserialize_port_key<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<PortKey, D::Error> {
    let key = String::deserialize(deserializer)?;
    PortKey::new(key.as_str().into()).map_err(|error| {
        serde::de::Error::custom(format!(
            "invalid port key `{key}`: {}",
            error.translate_data().message_key().as_str(),
        ))
    })
}

/// Accepts only strings and forwards them to `T`'s own deserializer.
struct UnitVariantVisitor<T> {
    /// The enum produced from the string.
    variant: PhantomData<T>,
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for UnitVariantVisitor<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a variant name string")
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        T::deserialize(StrDeserializer::new(value))
    }
}

/// Accepts only maps and forwards their entries to `T`'s own deserializer.
struct ObjectVisitor<T> {
    /// The type produced from the map.
    document: PhantomData<T>,
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for ObjectVisitor<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON object")
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        T::deserialize(MapAccessDeserializer::new(map))
    }
}

/// Accepts only objects whose keys occur once.
struct EntriesVisitor<T> {
    /// The type of each entry's value.
    value: PhantomData<T>,
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for EntriesVisitor<T> {
    type Value = BTreeMap<String, T>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON object with unique keys")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut entries = BTreeMap::new();
        while let Some(key) = map.next_key::<String>()? {
            match entries.entry(key) {
                Entry::Vacant(entry) => {
                    entry.insert(map.next_value()?);
                }
                Entry::Occupied(entry) => {
                    return Err(A::Error::custom(format!("duplicate key `{}`", entry.key())));
                }
            }
        }
        Ok(entries)
    }
}
