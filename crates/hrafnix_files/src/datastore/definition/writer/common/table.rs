use hrafnix_datastore::definition::TableColumnRequirement;
use hrafnix_shareable_string::ShareableString;
use serde::{Serialize, Serializer};

/// Version-1 adjacent-row requirement names.
#[derive(Debug, Serialize)]
#[serde(remote = "TableColumnRequirement", rename_all = "snake_case")]
pub(crate) enum RequirementWriterV1 {
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

/// Borrows table cell expressions without copying their string contents.
fn table_rows(rows: &[Vec<ShareableString>]) -> Vec<Vec<&str>> {
    rows.iter()
        .map(|row| row.iter().map(ShareableString::as_str).collect())
        .collect()
}

/// Serializes table rows as arrays of plain strings.
pub(crate) fn serialize_table_rows<S: Serializer>(
    rows: &[Vec<ShareableString>],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    table_rows(rows).serialize(serializer)
}

/// Serializes nullable rows, preserving the distinction between null and empty.
#[expect(
    clippy::ref_option,
    reason = "Serde passes a reference to the field type"
)]
pub(crate) fn serialize_optional_table_rows<S: Serializer>(
    rows: &Option<Vec<Vec<ShareableString>>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    rows.as_deref().map(table_rows).serialize(serializer)
}
