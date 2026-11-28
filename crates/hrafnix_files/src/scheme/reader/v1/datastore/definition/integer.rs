use crate::scheme::reader::v1::datastore::definition::common::constraints::ConstraintIntDefinitionReaderV1;
use hrafnix_datastore::definition::IntegerDefinition;
use serde::Deserialize;

/// Integer expression and bounds.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IntegerDefinitionReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// Integer bounds.
    constraint: ConstraintIntDefinitionReaderV1,
}

impl From<IntegerDefinitionReaderV1> for IntegerDefinition {
    fn from(integer: IntegerDefinitionReaderV1) -> Self {
        IntegerDefinition::new_with_constraint_and_default(
            integer.description,
            integer.constraint.into(),
            integer.default,
        )
    }
}
