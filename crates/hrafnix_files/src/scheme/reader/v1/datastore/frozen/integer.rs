use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::integer::IntegerDefinitionReaderV1;
use hrafnix_datastore::frozen::IntegerFrozen;
use serde::Deserialize;

/// Version-1 frozen integer definition and unevaluated expression.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IntegerReaderV1 {
    /// Integer metadata, default expression, and bounds.
    #[serde(deserialize_with = "deserialize_object")]
    definition: IntegerDefinitionReaderV1,
    /// Current unevaluated integer expression.
    value: String,
}

impl From<IntegerReaderV1> for IntegerFrozen {
    fn from(integer: IntegerReaderV1) -> Self {
        IntegerFrozen::new_with_value(integer.definition.into(), integer.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::{IntegerConstraint, IntegerDefinition};
    use serde_json::{Value, json};

    fn document() -> Value {
        json!({
            "definition": {
                "description": "Count",
                "default": "2",
                "constraint": {
                    "type": "range", "min": 0, "max": 10,
                    "min_inclusive": true, "max_inclusive": false
                }
            },
            "value": "1 + 2"
        })
    }

    #[test]
    fn reads_definition_bounds_and_expression() {
        let reader = serde_json::from_value::<RootObject<IntegerReaderV1>>(document()).unwrap();
        let integer = IntegerFrozen::from(reader.document);

        assert_eq!(integer.value(), "1 + 2");
        assert_eq!(
            integer.definition(),
            &IntegerDefinition::new_with_constraint_and_default(
                "Count",
                IntegerConstraint::range(0, 10, true, false),
                "2"
            )
        );
    }

    #[test]
    fn rejects_numeric_value_instead_of_expression() {
        let mut value = document();
        value["value"] = json!(3);
        assert!(serde_json::from_value::<RootObject<IntegerReaderV1>>(value).is_err());
    }
}
