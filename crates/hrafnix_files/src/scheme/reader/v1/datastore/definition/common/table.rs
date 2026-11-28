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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn reads_every_column_requirement() {
        for (name, expected) in [
            ("none", TableColumnRequirement::None),
            (
                "constant_or_increasing",
                TableColumnRequirement::ConstantOrIncreasing,
            ),
            ("increasing", TableColumnRequirement::Increasing),
            (
                "constant_or_decreasing",
                TableColumnRequirement::ConstantOrDecreasing,
            ),
            ("decreasing", TableColumnRequirement::Decreasing),
            ("constant", TableColumnRequirement::Constant),
        ] {
            let requirement =
                serde_json::from_value::<RequirementDefinitionReaderV1>(json!(name)).unwrap();
            assert_eq!(
                TableColumnRequirement::from(requirement),
                expected,
                "{name}"
            );
        }
    }

    #[test]
    fn rejects_unknown_requirement() {
        let error =
            serde_json::from_value::<RequirementDefinitionReaderV1>(json!("unknown")).unwrap_err();
        assert!(error.to_string().contains("unknown variant `unknown`"));
    }
}
