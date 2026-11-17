use crate::datastore::definition::reader::v1::common::constraints::ConstraintIntReaderV1;
use hrafnix_datastore::definition::IntegerDefinition;
use serde::Deserialize;

/// Integer expression and bounds.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IntegerReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// Integer bounds.
    constraint: ConstraintIntReaderV1,
}

impl From<IntegerReaderV1> for IntegerDefinition {
    fn from(integer: IntegerReaderV1) -> Self {
        IntegerDefinition::new_with_constraint_and_default(
            integer.description,
            integer.constraint.into(),
            integer.default,
        )
    }
}
