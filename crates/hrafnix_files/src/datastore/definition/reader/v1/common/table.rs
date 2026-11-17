use hrafnix_datastore::definition::TableColumnRequirement;
use serde::Deserialize;
use serde::de::Error;

/// Rejects default rows whose width differs from the table's column count.
pub(crate) fn validate_default_rows<E: Error>(
    rows: &[Vec<String>],
    column_count: usize,
) -> Result<(), E> {
    rows.iter()
        .enumerate()
        .find(|(_, row)| row.len() != column_count)
        .map_or(Ok(()), |(index, row)| {
            Err(E::custom(format!(
                "default row {index} has {} values but the table has {column_count} columns",
                row.len()
            )))
        })
}

/// Adjacent-row requirements accepted by version-1 tables.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RequirementReaderV1 {
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

impl From<RequirementReaderV1> for TableColumnRequirement {
    fn from(requirement: RequirementReaderV1) -> Self {
        match requirement {
            RequirementReaderV1::None => Self::None,
            RequirementReaderV1::ConstantOrIncreasing => Self::ConstantOrIncreasing,
            RequirementReaderV1::Increasing => Self::Increasing,
            RequirementReaderV1::ConstantOrDecreasing => Self::ConstantOrDecreasing,
            RequirementReaderV1::Decreasing => Self::Decreasing,
            RequirementReaderV1::Constant => Self::Constant,
        }
    }
}
