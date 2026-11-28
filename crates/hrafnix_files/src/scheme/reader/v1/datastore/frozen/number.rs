use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::number::NumberDefinitionReaderV1;
use hrafnix_datastore::frozen::NumberFrozen;
use serde::Deserialize;

/// Version-1 frozen number definition and unevaluated expression.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NumberReaderV1 {
    /// Numeric metadata, default expression, and bounds.
    #[serde(deserialize_with = "deserialize_object")]
    definition: NumberDefinitionReaderV1,
    /// Current unevaluated numeric expression.
    value: String,
}

impl From<NumberReaderV1> for NumberFrozen {
    fn from(number: NumberReaderV1) -> Self {
        NumberFrozen::new_with_value(number.definition.into(), number.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::{NumberConstraint, NumberDefinition};
    use serde_json::{Value, json};

    fn document() -> Value {
        json!({
            "definition": {
                "description": "Ratio",
                "default": "0.5",
                "constraint": {"type": "min", "min": 0.0, "inclusive": false}
            },
            "value": "1 / 3"
        })
    }

    #[test]
    fn reads_definition_bounds_and_expression() {
        let reader = serde_json::from_value::<RootObject<NumberReaderV1>>(document()).unwrap();
        let number = NumberFrozen::from(reader.document);

        assert_eq!(number.value(), "1 / 3");
        assert_eq!(
            number.definition(),
            &NumberDefinition::new_with_constraint_and_default(
                "Ratio",
                NumberConstraint::min(0.0, false),
                "0.5"
            )
        );
    }

    #[test]
    fn rejects_numeric_value_instead_of_expression() {
        let mut value = document();
        value["value"] = json!(0.5);
        assert!(serde_json::from_value::<RootObject<NumberReaderV1>>(value).is_err());
    }
}
