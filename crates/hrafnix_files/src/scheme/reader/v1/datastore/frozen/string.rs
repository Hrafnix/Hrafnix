use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::string::StringDefinitionReaderV1;
use hrafnix_datastore::frozen::StringFrozen;
use serde::Deserialize;

/// Version-1 frozen string definition and unevaluated expression.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StringReaderV1 {
    /// String metadata and default expression.
    #[serde(deserialize_with = "deserialize_object")]
    definition: StringDefinitionReaderV1,
    /// Current unevaluated string expression.
    value: String,
}

impl From<StringReaderV1> for StringFrozen {
    fn from(string: StringReaderV1) -> Self {
        StringFrozen::new_with_value(string.definition.into(), string.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use hrafnix_datastore::definition::StringDefinition;
    use serde_json::{Value, json};

    fn document() -> Value {
        json!({
            "definition": {"description": "Label", "default": "untitled"},
            "value": "sample \"A\"\nsecond line"
        })
    }

    #[test]
    fn reads_definition_and_string_contents() {
        let reader = serde_json::from_value::<RootObject<StringReaderV1>>(document()).unwrap();
        let string = StringFrozen::from(reader.document);

        assert_eq!(string.value(), "sample \"A\"\nsecond line");
        assert_eq!(
            string.definition(),
            &StringDefinition::new_with_default("Label", "untitled")
        );
    }

    #[test]
    fn rejects_missing_value() {
        let mut value = document();
        value.as_object_mut().unwrap().remove("value");
        assert!(serde_json::from_value::<RootObject<StringReaderV1>>(value).is_err());
    }
}
