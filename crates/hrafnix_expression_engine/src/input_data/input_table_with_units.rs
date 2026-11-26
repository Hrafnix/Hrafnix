use hrafnix_datastore::definition::TableWithUnitsDefinition;
use hrafnix_shareable_string::ShareableString;

/// Represents input data for a table, including its definition and the associated data.
#[derive(Debug, Clone, PartialEq)]
pub struct TableWithUnitsInputData {
    /// The definition that describes the columns and constraints of this table.
    definition: TableWithUnitsDefinition,
    /// The name of the parameter variable that holds the row index during cell evaluation,
    /// or an empty string if no such parameter is used.
    parameter: ShareableString,
    /// The names of the units associated with each column in the table.
    units: Vec<ShareableString>,
    /// The raw cell data, one inner `Vec<ShareableString>` per row.
    data: Vec<Vec<ShareableString>>,
}

impl TableWithUnitsInputData {
    /// Creates a new `TableWithUnitsInputData` with the given definition, parameter name, and raw data.
    pub(crate) const fn new(
        definition: TableWithUnitsDefinition,
        parameter: ShareableString,
        units: Vec<ShareableString>,
        data: Vec<Vec<ShareableString>>,
    ) -> Self {
        Self {
            definition,
            parameter,
            units,
            data,
        }
    }

    /// Returns a reference to the definition of the table input data.
    #[must_use]
    pub const fn definition(&self) -> &TableWithUnitsDefinition {
        &self.definition
    }

    /// Returns a reference to the units of the table input data.
    #[must_use]
    pub const fn units(&self) -> &[ShareableString] {
        self.units.as_slice()
    }

    /// Returns a reference to the data of the table input data.
    #[must_use]
    pub const fn data(&self) -> &[Vec<ShareableString>] {
        self.data.as_slice()
    }

    /// Returns a reference to the parameter name of the table input data.
    ///
    /// When non-empty, this name is bound to the current row index while each
    /// row's cell expressions are evaluated.
    #[must_use]
    pub const fn parameter(&self) -> &ShareableString {
        &self.parameter
    }

    /// Interns this value's strings in `store` in place, preserving its contents.
    pub fn launder(&mut self, store: &hrafnix_shareable_string::SharedStringStore) {
        self.definition.launder(store);
        self.parameter = store.launder(&self.parameter);
        for row in &mut self.data {
            for value in row {
                *value = store.launder(&*value);
            }
        }
        for unit in &mut self.units {
            *unit = store.launder(&*unit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::NumberWithUnitsDefinition;
    use hrafnix_keys::store_key;
    use hrafnix_shareable_string::SharedStringStore;
    use hrafnix_units::UnitId;

    #[test]
    fn laundering_interns_table_cells_parameter_and_units_in_place() {
        let store = SharedStringStore::new();
        for text in ["Table", "Column", "length", "row", "m", "42"] {
            store.get(text);
        }
        let mut data = TableWithUnitsInputData::new(
            TableWithUnitsDefinition::new(
                "Table",
                vec![(
                    store_key!("length"),
                    NumberWithUnitsDefinition::new("Column", UnitId::Length_Meter),
                )],
            ),
            ShareableString::new("row"),
            vec![ShareableString::new("m")],
            vec![vec![ShareableString::new("42")]],
        );
        let original = data.clone();
        data.launder(&store);
        assert_eq!(data, original);
        assert!(data.parameter().ptr_eq(&store.get("row")));
        assert!(data.units()[0].ptr_eq(&store.get("m")));
        assert!(data.data()[0][0].ptr_eq(&store.get("42")));
        assert!(
            data.definition()
                .description_ref()
                .ptr_eq(&store.get("Table"))
        );
        for (key, definition) in data.definition().iter() {
            assert!(key.as_shareable_string().ptr_eq(&store.get("length")));
            assert!(definition.description_ref().ptr_eq(&store.get("Column")));
        }
        assert!(!original.data()[0][0].ptr_eq(&store.get("42")));
    }
}
