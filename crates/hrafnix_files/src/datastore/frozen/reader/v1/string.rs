use crate::common::deserialize::deserialize_object;
use crate::datastore::definition::reader::v1::string::StringDefinitionReaderV1;
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
