use hrafnix_keys::{ComponentKey, GlobalKey, ParameterKey, PortKey, StoreKey, VariableKey};
use serde::de::value::{MapAccessDeserializer, StrDeserializer, StringDeserializer};
use serde::de::{DeserializeSeed, Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::{BTreeMap, BTreeSet, btree_map::Entry};
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

/// An array element identified by a key that must be unique within its array.
pub(crate) trait UniqueKey {
    /// Human-readable key name used in duplicate-key errors.
    const KEY_NAME: &'static str;

    /// Returns the element's identifying key.
    fn unique_key(&self) -> &StoreKey;
}

/// Deserializes an array of objects, rejecting positional arrays and duplicate keys.
pub(crate) fn deserialize_unique_objects<
    'de,
    T: Deserialize<'de> + UniqueKey,
    D: Deserializer<'de>,
>(
    deserializer: D,
) -> Result<Vec<T>, D::Error> {
    let items = deserialize_objects::<T, D>(deserializer)?;
    let mut keys = BTreeSet::new();
    for item in &items {
        let key = item.unique_key();
        if !keys.insert(key) {
            return Err(D::Error::custom(format!(
                "duplicate {} `{key}`",
                T::KEY_NAME
            )));
        }
    }
    Ok(items)
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

/// Deserializes a plain string through the shared component-key validator.
pub(crate) fn deserialize_component_key<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<ComponentKey, D::Error> {
    let key = String::deserialize(deserializer)?;
    ComponentKey::new(key.as_str().into()).map_err(|error| {
        serde::de::Error::custom(format!(
            "invalid component key `{key}`: {}",
            error.translate_data().message_key().as_str(),
        ))
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

/// Deserializes a plain string through the shared store-key validator.
pub(crate) fn deserialize_store_key<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<StoreKey, D::Error> {
    let key = String::deserialize(deserializer)?;
    StoreKey::new(key.as_str().into()).map_err(|error| {
        serde::de::Error::custom(format!(
            "invalid store key `{key}`: {}",
            error.translate_data().message_key().as_str(),
        ))
    })
}

/// Deserializes a plain string through the shared global-key validator.
pub(crate) fn deserialize_global_key<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<GlobalKey, D::Error> {
    let key = String::deserialize(deserializer)?;
    GlobalKey::new(key.as_str().into()).map_err(|error| {
        serde::de::Error::custom(format!(
            "invalid global key `{key}`: {}",
            error.translate_data().message_key().as_str(),
        ))
    })
}

/// Deserializes a plain string through the shared parameter-key validator.
pub(crate) fn deserialize_parameter_key<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<ParameterKey, D::Error> {
    let key = String::deserialize(deserializer)?;
    ParameterKey::new(key.as_str().into()).map_err(|error| {
        serde::de::Error::custom(format!(
            "invalid parameter key `{key}`: {}",
            error.translate_data().message_key().as_str(),
        ))
    })
}

/// Deserializes a plain string through the shared variable-key validator.
pub(crate) fn deserialize_variable_key<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<VariableKey, D::Error> {
    let key = String::deserialize(deserializer)?;
    VariableKey::new(key.as_str().into()).map_err(|error| {
        serde::de::Error::custom(format!(
            "invalid variable key `{key}`: {}",
            error.translate_data().message_key().as_str(),
        ))
    })
}

/// Requires an object's `type` tag to be a string before tagged-enum decoding.
///
/// Serde also accepts numeric variant indexes by default. This adapter preserves
/// the original map entries, including duplicates, while rejecting that form.
pub(crate) struct StringTagDeserializer<D>(pub(crate) D);

impl<'de, D: Deserializer<'de>> Deserializer<'de> for StringTagDeserializer<D> {
    type Error = D::Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_map(StringTagVisitor { visitor })
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct
        map struct enum identifier ignored_any
    }
}

/// Wraps map access before passing it to a tagged enum's generated visitor.
struct StringTagVisitor<V> {
    /// Generated enum visitor.
    visitor: V,
}

impl<'de, V: Visitor<'de>> Visitor<'de> for StringTagVisitor<V> {
    type Value = V::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.visitor.expecting(formatter)
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        self.visitor.visit_map(StringTagMapAccess {
            map,
            is_type: false,
        })
    }
}

/// Checks tag values without buffering or dropping any map entries.
struct StringTagMapAccess<A> {
    /// Original map entries.
    map: A,
    /// Whether the next value belongs to the `type` field.
    is_type: bool,
}

impl<'de, A: MapAccess<'de>> MapAccess<'de> for StringTagMapAccess<A> {
    type Error = A::Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        let Some(key) = self.map.next_key::<String>()? else {
            return Ok(None);
        };
        self.is_type = key == "type";
        seed.deserialize(StringDeserializer::<A::Error>::new(key))
            .map(Some)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        if self.is_type {
            let tag = self.map.next_value::<String>()?;
            seed.deserialize(StringDeserializer::<A::Error>::new(tag))
        } else {
            self.map.next_value_seed(seed)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        self.map.size_hint()
    }
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
