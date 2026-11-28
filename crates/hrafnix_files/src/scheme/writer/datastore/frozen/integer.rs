use crate::common::serialize::serialize_shareable_string;
use crate::scheme::writer::datastore::definition::integer::IntegerDefinitionWriterV1;
use hrafnix_datastore::frozen::IntegerFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen integer definition and unevaluated expression.
#[derive(Debug, Serialize)]
pub(crate) struct IntegerWriterV1 {
    /// Integer metadata, default expression, and bounds.
    definition: IntegerDefinitionWriterV1,
    /// Current unevaluated integer expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    value: ShareableString,
}

impl From<&IntegerFrozen> for IntegerWriterV1 {
    fn from(integer: &IntegerFrozen) -> Self {
        Self {
            definition: integer.definition().into(),
            value: integer.value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::{IntegerConstraint, IntegerDefinition};
    use serde_json::json;

    #[test]
    fn writes_bounds_default_and_current_expression() {
        let integer = IntegerFrozen::new_with_value(
            IntegerDefinition::new_with_constraint_and_default(
                "Count",
                IntegerConstraint::range(0, 10, true, false),
                "2",
            ),
            "1 + 2",
        );

        assert_eq!(
            serde_json::to_value(IntegerWriterV1::from(&integer)).unwrap(),
            json!({
                "definition": {
                    "description": "Count", "default": "2",
                    "constraint": {
                        "type": "range", "min": 0, "max": 10,
                        "min_inclusive": true, "max_inclusive": false
                    }
                },
                "value": "1 + 2"
            })
        );
    }
}
