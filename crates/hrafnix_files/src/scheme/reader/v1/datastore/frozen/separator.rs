use crate::common::deserialize::deserialize_object;
use crate::scheme::reader::v1::datastore::definition::separator::SeparatorDefinitionReaderV1;
use hrafnix_datastore::frozen::SeparatorFrozen;
use serde::Deserialize;

/// Version-1 frozen separator, which has metadata but no value.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SeparatorReaderV1 {
    /// Separator label.
    #[serde(deserialize_with = "deserialize_object")]
    definition: SeparatorDefinitionReaderV1,
}

impl From<SeparatorReaderV1> for SeparatorFrozen {
    fn from(separator: SeparatorReaderV1) -> Self {
        SeparatorFrozen::new(separator.definition.into())
    }
}
