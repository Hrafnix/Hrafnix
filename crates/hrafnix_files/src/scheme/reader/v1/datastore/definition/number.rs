use crate::scheme::reader::v1::datastore::definition::common::constraints::ConstraintNumberDefinitionReaderV1;
use hrafnix_datastore::definition::NumberDefinition;
use serde::Deserialize;

/// Numeric expression and bounds.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NumberDefinitionReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// Number bounds.
    constraint: ConstraintNumberDefinitionReaderV1,
}

impl From<NumberDefinitionReaderV1> for NumberDefinition {
    fn from(number: NumberDefinitionReaderV1) -> Self {
        NumberDefinition::new_with_constraint_and_default(
            number.description,
            number.constraint.into(),
            number.default,
        )
    }
}
