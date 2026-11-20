use hrafnix_datastore::definition::TableColumnRequirement;
use serde::Deserialize;

/// Adjacent-row requirements accepted by version-1 tables.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RequirementDefinitionReaderV1 {
    /// No relationship required between rows.
    None,
    /// Each value is at least the preceding value.
    ConstantOrIncreasing,
    /// Each value is greater than the preceding value.
    Increasing,
    /// Each value is at most the preceding value.
    ConstantOrDecreasing,
    /// Each value is less than the preceding value.
    Decreasing,
    /// Adjacent values are equal.
    Constant,
}

impl From<RequirementDefinitionReaderV1> for TableColumnRequirement {
    fn from(requirement: RequirementDefinitionReaderV1) -> Self {
        match requirement {
            RequirementDefinitionReaderV1::None => Self::None,
            RequirementDefinitionReaderV1::ConstantOrIncreasing => Self::ConstantOrIncreasing,
            RequirementDefinitionReaderV1::Increasing => Self::Increasing,
            RequirementDefinitionReaderV1::ConstantOrDecreasing => Self::ConstantOrDecreasing,
            RequirementDefinitionReaderV1::Decreasing => Self::Decreasing,
            RequirementDefinitionReaderV1::Constant => Self::Constant,
        }
    }
}
