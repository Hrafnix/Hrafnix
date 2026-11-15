use crate::datastore::definition::reader::v1::common::constraints::ConstraintNumberReaderV1;
use hrafnix_datastore::definition::NumberDefinition;
use serde::Deserialize;

/// Numeric expression and bounds.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NumberReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// Number bounds.
    constraint: ConstraintNumberReaderV1,
}

impl From<NumberReaderV1> for NumberDefinition {
    fn from(number: NumberReaderV1) -> Self {
        NumberDefinition::new_with_constraint_and_default(
            number.description,
            number.constraint.into(),
            number.default,
        )
    }
}
