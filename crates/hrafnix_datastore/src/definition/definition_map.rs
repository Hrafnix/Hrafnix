use crate::definition::{
    BooleanDefinition, ChoiceDefinition, FileDefinition, IntegerDefinition, NumberDefinition,
    NumberWithUnitsDefinition, StringDefinition, TableDefinition, TableWithUnitsDefinition,
    UnitDefinition,
};
use crate::traits::TreePrint;
use hrafnix_keys::store_key::StoreKey;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;
use std::sync::Arc;

/// A default value for an item in a map entry.
#[derive(Debug, Clone, PartialEq)]
pub enum MapItemDefault {
    /// A default value for a scalar map item.
    Scalar(ShareableString),
    /// Default rows for a table map item.
    Table(Vec<Vec<ShareableString>>),
}

impl MapItemDefault {
    /// Creates a scalar default value.
    #[must_use]
    pub fn scalar<S: Into<ShareableString>>(value: S) -> Self {
        Self::Scalar(value.into())
    }

    /// Creates a table default value.
    #[must_use]
    pub fn table<S: Into<ShareableString>>(rows: Vec<Vec<S>>) -> Self {
        Self::Table(
            rows.into_iter()
                .map(|row| row.into_iter().map(Into::into).collect())
                .collect(),
        )
    }

    /// Matches the item's kind and normalizes table row widths, inheriting mismatched defaults.
    fn normalized(self, definition: &MapItemDefinition) -> Self {
        match definition {
            MapItemDefinition::Boolean(item) => self.normalized_scalar(item.default_value_ref()),
            MapItemDefinition::Choice(item) => self.normalized_scalar(item.default_value_ref()),
            MapItemDefinition::File(item) => self.normalized_scalar(item.default_value_ref()),
            MapItemDefinition::Integer(item) => self.normalized_scalar(item.default_value_ref()),
            MapItemDefinition::Number(item) => self.normalized_scalar(item.default_value_ref()),
            MapItemDefinition::NumberWithUnits(item) => {
                self.normalized_scalar(item.default_value_ref())
            }
            MapItemDefinition::String(item) => self.normalized_scalar(item.default_value_ref()),
            MapItemDefinition::Table(item) => {
                self.normalized_table(item.count(), item.default_table())
            }
            MapItemDefinition::TableWithUnits(item) => {
                self.normalized_table(item.count(), item.default_table())
            }
            MapItemDefinition::Unit(item) => self.normalized_scalar(item.default_value_ref()),
        }
    }

    /// Replaces table defaults with the scalar item's default expression.
    fn normalized_scalar(self, default: &ShareableString) -> Self {
        match self {
            scalar @ Self::Scalar(_) => scalar,
            Self::Table(_) => Self::Scalar(default.clone()),
        }
    }

    /// Normalizes table rows or inherits the table's default rows for a scalar default.
    fn normalized_table(
        self,
        column_count: usize,
        default: Option<&[Vec<ShareableString>]>,
    ) -> Self {
        match self {
            Self::Table(rows) => Self::Table(
                rows.into_iter()
                    .map(|mut row| {
                        row.resize(column_count, ShareableString::default());
                        row
                    })
                    .collect(),
            ),
            Self::Scalar(_) => Self::Table(default.unwrap_or_default().to_vec()),
        }
    }

    /// Feeds this default value into a BLAKE3 hasher.
    fn update_hasher(&self, h: &mut blake3::Hasher) {
        match self {
            Self::Scalar(value) => {
                h.update(&[0x00]);
                h.update(&value.current_blake3_hash());
            }
            Self::Table(rows) => {
                h.update(&[0x01]);
                h.update(&u64::try_from(rows.len()).unwrap_or(u64::MAX).to_le_bytes());
                for row in rows {
                    h.update(&u64::try_from(row.len()).unwrap_or(u64::MAX).to_le_bytes());
                    for cell in row {
                        h.update(&cell.current_blake3_hash());
                    }
                }
            }
        }
    }

    /// Interns all strings in `store` in place, preserving values, hashes, and ordering.
    fn launder(&mut self, store: &SharedStringStore) {
        match self {
            Self::Scalar(value) => *value = store.launder(&*value),
            Self::Table(rows) => {
                for row in rows {
                    for value in row {
                        *value = store.launder(&*value);
                    }
                }
            }
        }
    }
}

/// Default values for one map entry, in the map definition's item order.
pub type MapEntryDefault = Vec<MapItemDefault>;

/// The definition of an item within a map entry.
#[derive(Debug, Clone, PartialEq)]
pub enum MapItemDefinition {
    /// A boolean parameter.
    Boolean(BooleanDefinition),
    /// A choice parameter.
    Choice(ChoiceDefinition),
    /// A file parameter.
    File(FileDefinition),
    /// An integer parameter.
    Integer(IntegerDefinition),
    /// A number parameter.
    Number(NumberDefinition),
    /// A number with units parameter.
    NumberWithUnits(NumberWithUnitsDefinition),
    /// A string parameter.
    String(StringDefinition),
    /// A table parameter.
    Table(TableDefinition),
    /// A table parameter with units.
    TableWithUnits(TableWithUnitsDefinition),
    /// A unit parameter.
    Unit(UnitDefinition),
}

impl From<BooleanDefinition> for MapItemDefinition {
    fn from(definition: BooleanDefinition) -> Self {
        Self::Boolean(definition)
    }
}

impl From<ChoiceDefinition> for MapItemDefinition {
    fn from(definition: ChoiceDefinition) -> Self {
        Self::Choice(definition)
    }
}

impl From<FileDefinition> for MapItemDefinition {
    fn from(definition: FileDefinition) -> Self {
        Self::File(definition)
    }
}

impl From<IntegerDefinition> for MapItemDefinition {
    fn from(definition: IntegerDefinition) -> Self {
        Self::Integer(definition)
    }
}

impl From<NumberDefinition> for MapItemDefinition {
    fn from(definition: NumberDefinition) -> Self {
        Self::Number(definition)
    }
}

impl From<StringDefinition> for MapItemDefinition {
    fn from(definition: StringDefinition) -> Self {
        Self::String(definition)
    }
}

impl From<TableDefinition> for MapItemDefinition {
    fn from(definition: TableDefinition) -> Self {
        Self::Table(definition)
    }
}

impl From<TableWithUnitsDefinition> for MapItemDefinition {
    fn from(definition: TableWithUnitsDefinition) -> Self {
        Self::TableWithUnits(definition)
    }
}

impl From<UnitDefinition> for MapItemDefinition {
    fn from(definition: UnitDefinition) -> Self {
        Self::Unit(definition)
    }
}

impl MapItemDefinition {
    /// Returns the pre-calculated BLAKE3 hash of the map item definition.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        match self {
            Self::Boolean(def) => def.hash(),
            Self::Choice(def) => def.hash(),
            Self::File(def) => def.hash(),
            Self::Integer(def) => def.hash(),
            Self::Number(def) => def.hash(),
            Self::NumberWithUnits(def) => def.hash(),
            Self::String(def) => def.hash(),
            Self::Table(def) => def.hash(),
            Self::TableWithUnits(def) => def.hash(),
            Self::Unit(def) => def.hash(),
        }
    }

    /// Returns true if values of `other` can safely replace values of `self` during a merge.
    ///
    /// Descriptions and defaults are ignored; the variant must match and its structure must be merge compatible.
    #[must_use]
    pub fn is_merge_compatible(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(_), Self::Boolean(_))
            | (Self::Integer(_), Self::Integer(_))
            | (Self::Number(_), Self::Number(_))
            | (Self::String(_), Self::String(_)) => true,
            (Self::Choice(a), Self::Choice(b)) => a.is_merge_compatible(b),
            (Self::File(a), Self::File(b)) => a.is_merge_compatible(b),
            (Self::NumberWithUnits(a), Self::NumberWithUnits(b)) => a.is_merge_compatible(b),
            (Self::Table(a), Self::Table(b)) => a.is_merge_compatible(b),
            (Self::TableWithUnits(a), Self::TableWithUnits(b)) => a.is_merge_compatible(b),
            (Self::Unit(a), Self::Unit(b)) => a.is_merge_compatible(b),
            _ => false,
        }
    }

    /// Interns all strings in `store` in place, preserving values, hashes, and ordering.
    pub fn launder(&mut self, store: &SharedStringStore) {
        match self {
            Self::Boolean(def) => def.launder(store),
            Self::Choice(def) => def.launder(store),
            Self::File(def) => def.launder(store),
            Self::Integer(def) => def.launder(store),
            Self::Number(def) => def.launder(store),
            Self::NumberWithUnits(def) => def.launder(store),
            Self::String(def) => def.launder(store),
            Self::Table(def) => def.launder(store),
            Self::TableWithUnits(def) => def.launder(store),
            Self::Unit(def) => def.launder(store),
        }
    }
}

impl PartialEq<&MapItemDefinition> for MapItemDefinition {
    fn eq(&self, other: &&MapItemDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<MapItemDefinition> for &MapItemDefinition {
    fn eq(&self, other: &MapItemDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for MapItemDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        match self {
            MapItemDefinition::Boolean(boolean) => boolean.tree_print(f, label, prefix, last),
            MapItemDefinition::Choice(choice) => choice.tree_print(f, label, prefix, last),
            MapItemDefinition::File(file) => file.tree_print(f, label, prefix, last),
            MapItemDefinition::Integer(integer) => integer.tree_print(f, label, prefix, last),
            MapItemDefinition::Number(number) => number.tree_print(f, label, prefix, last),
            MapItemDefinition::NumberWithUnits(number_with_units) => {
                number_with_units.tree_print(f, label, prefix, last)
            }
            MapItemDefinition::String(string) => string.tree_print(f, label, prefix, last),
            MapItemDefinition::Table(table) => table.tree_print(f, label, prefix, last),
            MapItemDefinition::TableWithUnits(table_with_units) => {
                table_with_units.tree_print(f, label, prefix, last)
            }
            MapItemDefinition::Unit(unit) => unit.tree_print(f, label, prefix, last),
        }
    }
}

/// Definition for a map parameter where keys are strings and values follow a fixed schema of
/// named `MapItemDefinition`s.
#[derive(Debug, Clone)]
pub struct MapDefinition {
    /// Human-readable description of this map parameter.
    description: ShareableString,
    /// Keys in insertion order, used to preserve deterministic iteration.
    ordered_keys: Vec<StoreKey>,
    /// Schema of the map entries, keyed by item name.
    item_type: Arc<BTreeMap<StoreKey, MapItemDefinition>>,
    /// Optional default map entries.
    default_map: Option<BTreeMap<StoreKey, MapEntryDefault>>,
    /// Pre-computed BLAKE3 hash of this definition.
    hash: [u8; 32],
}

impl MapDefinition {
    /// Creates a new `MapDefinition` with a description and a list of entry items.
    ///
    /// If duplicate keys are provided, the last occurrence will be used, and the order of the keys will
    /// reflect the order of their last occurrence.
    pub fn new<S1: Into<ShareableString>, K: Into<StoreKey>, I: Into<MapItemDefinition>>(
        description: S1,
        item_type: Vec<(K, I)>,
    ) -> Self {
        let mut items = BTreeMap::new();
        let mut ordered_keys = Vec::new();
        for (k, v) in item_type {
            let key = k.into();
            if items.contains_key(&key) {
                ordered_keys.retain(|existing_key| existing_key != &key);
            }
            ordered_keys.push(key.clone());
            items.insert(key, v.into());
        }
        let mut s = Self {
            description: description.into(),
            ordered_keys,
            item_type: Arc::new(items),
            default_map: None,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `MapDefinition` with default map entries.
    ///
    /// Items omitted from a default entry inherit the default from their item definition.
    /// Extra defaults are truncated to the resulting item count. Defaults whose scalar/table
    /// kind does not match their item inherit that item's default without shifting later values.
    /// Table rows are padded or truncated to the corresponding table's column count.
    pub fn new_with_default<
        S1: Into<ShareableString>,
        K: Into<StoreKey>,
        I: Into<MapItemDefinition>,
    >(
        description: S1,
        item_type: Vec<(K, I)>,
        default_map: BTreeMap<StoreKey, MapEntryDefault>,
    ) -> Self {
        let mut definition = Self::new(description, item_type);
        let item_definitions: Vec<_> = definition.iter().map(|(_, item)| item).collect();
        definition.default_map = Some(
            default_map
                .into_iter()
                .map(|(key, entry)| {
                    let entry = entry
                        .into_iter()
                        .zip(&item_definitions)
                        .map(|(default, item)| default.normalized(item))
                        .collect();
                    (key, entry)
                })
                .collect(),
        );
        definition.update_hash();
        definition
    }

    /// Computes and stores the BLAKE3 hash of this definition.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        h.update(&[0x01]);
        h.update(b"MapDefinition");
        h.update(&self.description.current_blake3_hash());
        h.update(
            &u64::try_from(self.ordered_keys.len())
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );
        for (key, item) in self.iter() {
            h.update(&key.current_blake3_hash());
            h.update(&item.hash());
        }

        match &self.default_map {
            None => {
                h.update(&[0x00]);
            }
            Some(entries) => {
                h.update(&[0x01]);
                h.update(
                    &u64::try_from(entries.len())
                        .unwrap_or(u64::MAX)
                        .to_le_bytes(),
                );
                for (entry_key, entry) in entries {
                    h.update(&entry_key.current_blake3_hash());
                    h.update(&u64::try_from(entry.len()).unwrap_or(u64::MAX).to_le_bytes());
                    for item_default in entry {
                        item_default.update_hasher(&mut h);
                    }
                }
            }
        }

        self.hash = *h.finalize().as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the definition.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns the description of the map.
    #[must_use]
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the map item definition for the specified key.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&MapItemDefinition> {
        self.item_type.get(&key.into())
    }

    /// Returns a reference to the map item definition for the specified key string.
    #[must_use]
    pub fn get_str(&self, key: &str) -> Option<&MapItemDefinition> {
        self.item_type.get(key)
    }

    /// Returns true if the map's entry schema contains an item with the specified key.
    pub fn contains_key<S: Into<ShareableString>>(&self, key: S) -> bool {
        self.item_type.contains_key(&key.into())
    }

    /// Returns an iterator over the keys of the map's entry schema.
    pub fn keys(&self) -> impl Iterator<Item = &StoreKey> {
        self.ordered_keys.iter()
    }

    /// Returns true if the map's entry schema contains an item with the specified key string.
    #[must_use]
    pub fn contains_key_str(&self, key: &str) -> bool {
        self.item_type.contains_key(key)
    }

    /// Returns an iterator over the map's entry item definitions.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &MapItemDefinition)> {
        self.ordered_keys
            .iter()
            .filter_map(move |key| self.item_type.get(key).map(|v| (key, v)))
    }

    /// Returns the number of items in the map's entry schema.
    #[must_use]
    pub fn count(&self) -> usize {
        self.item_type.len()
    }

    /// Returns a reference to the map's entry item type.
    #[must_use]
    pub fn item_type(&self) -> &BTreeMap<StoreKey, MapItemDefinition> {
        &self.item_type
    }

    /// Returns the default map entries, if configured.
    #[must_use]
    pub const fn default_map(&self) -> Option<&BTreeMap<StoreKey, MapEntryDefault>> {
        self.default_map.as_ref()
    }

    /// Returns a reference to the description.
    #[must_use]
    pub const fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns true if values of `other` can safely replace values of `self` during a merge.
    ///
    /// Descriptions and defaults are ignored; entry items must match in order and be merge compatible.
    #[must_use]
    pub fn is_merge_compatible(&self, other: &Self) -> bool {
        self.ordered_keys == other.ordered_keys
            && self
                .iter()
                .zip(other.iter())
                .all(|((_, a), (_, b))| a.is_merge_compatible(b))
    }

    /// Interns all strings in `store` in place, preserving values, hashes, and ordering.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.description = store.launder(&self.description);
        for value in &mut self.ordered_keys {
            value.launder(store);
        }
        let item_type = Arc::make_mut(&mut self.item_type);
        *item_type = std::mem::take(item_type)
            .into_iter()
            .map(|(mut key, mut value)| {
                key.launder(store);
                value.launder(store);
                (key, value)
            })
            .collect();
        if let Some(entries) = &mut self.default_map {
            *entries = std::mem::take(entries)
                .into_iter()
                .map(|(mut key, mut entry)| {
                    key.launder(store);
                    for value in &mut entry {
                        value.launder(store);
                    }
                    (key, entry)
                })
                .collect();
        }
    }
}

impl PartialEq for MapDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
            && self.description == other.description
            && self.ordered_keys == other.ordered_keys
            && self.item_type == other.item_type
            && self.default_map == other.default_map
    }
}

impl PartialEq<&MapDefinition> for MapDefinition {
    fn eq(&self, other: &&MapDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<MapDefinition> for &MapDefinition {
    fn eq(&self, other: &MapDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for MapDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Map",
            prefix,
            Self::branch_char(last),
            label,
            self.description(),
        )?;

        let child_prefix = Self::child_prefix(prefix, last);

        let mut item_iter = self.item_type.iter().peekable();

        while let Some((key, item)) = item_iter.next() {
            let is_last = item_iter.peek().is_none();
            item.tree_print(f, key.as_str(), &child_prefix, is_last)?;
        }

        Ok(())
    }
}
