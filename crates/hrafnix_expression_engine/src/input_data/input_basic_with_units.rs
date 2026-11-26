use crate::BasicDefinition;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};

/// Represents basic input data in the system.
///
/// The `BasicInputData` struct is used to encapsulate
/// the definition of a basic input data item.
#[derive(Debug, Clone, PartialEq)]
pub struct BasicInputWithUnitsData {
    /// The definition describing valid values for this input item.
    definition: BasicDefinition,
    /// The raw string value provided by the user.
    data: ShareableString,
    /// The raw string value for the units provided by the user.
    units: ShareableString,
}

impl BasicInputWithUnitsData {
    /// Creates a new `BasicInputWithUnitsData` with the given `definition`, raw `data`, and `units`.
    pub(crate) const fn new(
        definition: BasicDefinition,
        data: ShareableString,
        units: ShareableString,
    ) -> Self {
        Self {
            definition,
            data,
            units,
        }
    }

    /// Returns a reference to the definition of the basic input data.
    #[must_use]
    pub const fn definition(&self) -> &BasicDefinition {
        &self.definition
    }

    /// Returns a reference to the data of the basic input data.
    #[must_use]
    pub const fn data(&self) -> &ShareableString {
        &self.data
    }

    /// Returns a reference to the units of the basic input data.
    #[must_use]
    pub const fn units(&self) -> &ShareableString {
        &self.units
    }

    /// Interns this value's strings in `store` in place, preserving its contents.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.definition.launder(store);
        self.data = store.launder(&self.data);
        self.units = store.launder(&self.units);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::NumberWithUnitsDefinition;
    use hrafnix_units::UnitId;

    #[test]
    fn laundering_interns_values_and_units_in_place() {
        let store = SharedStringStore::new();
        let value = store.get("42");
        let units = store.get("m");
        let description = store.get("Length");
        let mut data = BasicInputWithUnitsData::new(
            BasicDefinition::NumberWithUnits(NumberWithUnitsDefinition::new(
                "Length",
                UnitId::Length_Meter,
            )),
            ShareableString::new("42"),
            ShareableString::new("m"),
        );
        let original = data.clone();
        data.launder(&store);
        assert_eq!(data, original);
        assert!(data.data().ptr_eq(&value));
        assert!(data.units().ptr_eq(&units));
        let BasicDefinition::NumberWithUnits(definition) = data.definition() else {
            panic!("expected unit-aware definition");
        };
        assert!(definition.description_ref().ptr_eq(&description));
        assert!(!original.data().ptr_eq(&value));
    }
}
