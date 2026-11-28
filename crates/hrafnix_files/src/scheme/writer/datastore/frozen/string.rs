use crate::common::serialize::serialize_shareable_string;
use crate::scheme::writer::datastore::definition::string::StringDefinitionWriterV1;
use hrafnix_datastore::frozen::StringFrozen;
use hrafnix_shareable_string::ShareableString;
use serde::Serialize;

/// Version-1 frozen string definition and unevaluated expression.
#[derive(Debug, Serialize)]
pub(crate) struct StringWriterV1 {
    /// String metadata and default expression.
    definition: StringDefinitionWriterV1,
    /// Current unevaluated string expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    value: ShareableString,
}

impl From<&StringFrozen> for StringWriterV1 {
    fn from(string: &StringFrozen) -> Self {
        Self {
            definition: string.definition().into(),
            value: string.value(),
        }
    }
}
