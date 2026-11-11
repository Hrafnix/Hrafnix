use serde::de::value::{MapAccessDeserializer, StrDeserializer};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
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

/// Deserializes a unit-only enum from a plain string, rejecting the object form
/// (such as `{"Variant": null}`) that Serde otherwise accepts.
pub(crate) fn deserialize_unit_variant<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<T, D::Error> {
    deserializer.deserialize_str(UnitVariantVisitor {
        variant: PhantomData,
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
