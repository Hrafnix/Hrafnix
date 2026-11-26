use hrafnix_datastore::definition::TableDefinition;
use hrafnix_shareable_string::ShareableString;

/// Represents input data for a table, including its definition and the associated data.
#[derive(Debug, Clone, PartialEq)]
pub struct TableInputData {
    /// The definition that describes the columns and constraints of this table.
    definition: TableDefinition,
    /// The name of the parameter variable that holds the row index during cell evaluation,
    /// or an empty string if no such parameter is used.
    parameter: ShareableString,
    /// The raw cell data, one inner `Vec<ShareableString>` per row.
    data: Vec<Vec<ShareableString>>,
}

impl TableInputData {
    /// Creates a new `TableInputData` with the given definition, parameter name, and raw data.
    pub(crate) const fn new(
        definition: TableDefinition,
        parameter: ShareableString,
        data: Vec<Vec<ShareableString>>,
    ) -> Self {
        Self {
            definition,
            parameter,
            data,
        }
    }

    /// Returns a reference to the definition of the table input data.
    #[must_use]
    pub const fn definition(&self) -> &TableDefinition {
        &self.definition
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
    }
}
