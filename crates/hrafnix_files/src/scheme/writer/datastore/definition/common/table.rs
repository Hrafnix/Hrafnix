use hrafnix_datastore::definition::TableColumnRequirement;
use hrafnix_shareable_string::ShareableString;
use serde::{Serialize, Serializer};

/// Version-1 adjacent-row requirement names.
#[derive(Debug, Serialize)]
#[serde(remote = "TableColumnRequirement", rename_all = "snake_case")]
pub(crate) enum RequirementDefinitionWriterV1 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json, value::Serializer};

    #[test]
    fn writes_every_requirement_as_a_plain_string() {
        for (requirement, name) in [
            (TableColumnRequirement::None, "none"),
            (
                TableColumnRequirement::ConstantOrIncreasing,
                "constant_or_increasing",
            ),
            (TableColumnRequirement::Increasing, "increasing"),
            (
                TableColumnRequirement::ConstantOrDecreasing,
                "constant_or_decreasing",
            ),
            (TableColumnRequirement::Decreasing, "decreasing"),
            (TableColumnRequirement::Constant, "constant"),
        ] {
            assert_eq!(
                RequirementDefinitionWriterV1::serialize(&requirement, Serializer).unwrap(),
                json!(name)
            );
        }
    }

    #[test]
    fn writes_row_major_plain_strings() {
        let rows = vec![
            vec![
                ShareableString::from("1 + 2"),
                ShareableString::from("héllo\n\"world\""),
            ],
            vec![ShareableString::from("4"), ShareableString::from("")],
        ];
        let expected = json!([["1 + 2", "héllo\n\"world\""], ["4", ""]]);
        assert_eq!(
            table_rows(&rows),
            vec![vec!["1 + 2", "héllo\n\"world\""], vec!["4", ""]]
        );
        assert_eq!(serialize_table_rows(&rows, Serializer).unwrap(), expected);
        assert_eq!(
            serialize_optional_table_rows(&Some(rows), Serializer).unwrap(),
            expected
        );
    }

    #[test]
    fn distinguishes_absent_empty_and_empty_row_defaults() {
        assert_eq!(
            serialize_optional_table_rows(&None, Serializer).unwrap(),
            Value::Null
        );
        assert_eq!(
            serialize_optional_table_rows(&Some(Vec::new()), Serializer).unwrap(),
            json!([])
        );
        assert_eq!(serialize_table_rows(&[], Serializer).unwrap(), json!([]));
        assert_eq!(
            serialize_optional_table_rows(&Some(vec![Vec::new()]), Serializer).unwrap(),
            json!([[]])
        );
    }
}
