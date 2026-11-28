use crate::common::serialize::serialize_shareable_string;
use crate::scheme::writer::datastore::definition::number::NumberDefinitionWriterV1;
use hrafnix_datastore::frozen::NumberFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen number definition and unevaluated expression.
#[derive(Debug, Serialize)]
pub(crate) struct NumberWriterV1 {
    /// Numeric metadata, default expression, and bounds.
    definition: NumberDefinitionWriterV1,
    /// Current unevaluated numeric expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    value: ShareableString,
}

impl From<&NumberFrozen> for NumberWriterV1 {
    fn from(number: &NumberFrozen) -> Self {
        Self {
            definition: number.definition().into(),
            value: number.value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_datastore::definition::{NumberConstraint, NumberDefinition};
    use serde_json::json;

    #[test]
    fn writes_bounds_default_and_current_expression() {
        let number = NumberFrozen::new_with_value(
            NumberDefinition::new_with_constraint_and_default(
                "Ratio",
                NumberConstraint::min(0.0, false),
                "0.5",
            ),
            "1 / 3",
        );

        assert_eq!(
            serde_json::to_value(NumberWriterV1::from(&number)).unwrap(),
            json!({
                "definition": {
                    "description": "Ratio", "default": "0.5",
                    "constraint": {"type": "min", "min": 0.0, "inclusive": false}
                },
                "value": "1 / 3"
            })
        );
    }
}
