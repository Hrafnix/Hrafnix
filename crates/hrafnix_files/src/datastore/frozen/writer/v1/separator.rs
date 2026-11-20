use crate::datastore::definition::writer::separator::SeparatorDefinitionWriterV1;
use hrafnix_datastore::frozen::SeparatorFrozen;
use serde::Serialize;

/// Version-1 frozen separator, which has metadata but no value.
#[derive(Debug, Serialize)]
pub(crate) struct SeparatorWriterV1 {
    /// Separator label.
    definition: SeparatorDefinitionWriterV1,
}

impl From<&SeparatorFrozen> for SeparatorWriterV1 {
    fn from(separator: &SeparatorFrozen) -> Self {
        Self {
            definition: separator.definition().into(),
        }
    }
}
