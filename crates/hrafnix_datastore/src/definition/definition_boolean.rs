use crate::traits::TreePrint;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};

/// Definition for a boolean parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanDefinition {
    /// Human-readable description of this boolean parameter.
    description: ShareableString,
    /// Label displayed when the value is `true`.
    true_description: ShareableString,
    /// Label displayed when the value is `false`.
    false_description: ShareableString,
    /// Default value for this boolean parameter.
    default_value: ShareableString,
}

impl BooleanDefinition {
    /// Creates a new `BooleanDefinition` with the specified description.
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        Self {
            description: description.into(),
            true_description: ShareableString::new("True"),
            false_description: ShareableString::new("False"),
            default_value: ShareableString::new(""),
        }
    }

    /// Creates a new `BooleanDefinition` with the specified description and default value.
    pub fn new_with_default<S: Into<ShareableString>>(description: S, default_value: bool) -> Self {
        Self {
            description: description.into(),
            true_description: ShareableString::new("True"),
            false_description: ShareableString::new("False"),
            default_value: ShareableString::new(if default_value { "true" } else { "false" }),
        }
    }

    /// Creates a boolean definition with custom labels and an unevaluated default expression.
    pub fn new_with_descriptions_and_default<
        S1: Into<ShareableString>,
        S2: Into<ShareableString>,
        S3: Into<ShareableString>,
        S4: Into<ShareableString>,
    >(
        description: S1,
        true_description: S2,
        false_description: S3,
        default_value: S4,
    ) -> Self {
        Self {
            description: description.into(),
            true_description: true_description.into(),
            false_description: false_description.into(),
            default_value: default_value.into(),
        }
    }

    /// Returns a reference to the label for the true state.
    #[must_use]
    pub const fn true_description_ref(&self) -> &ShareableString {
        &self.true_description
    }

    /// Returns a reference to the label for the false state.
    #[must_use]
    pub const fn false_description_ref(&self) -> &ShareableString {
        &self.false_description
    }

    /// Returns a vector of IDs for the choices.
    #[must_use]
    pub fn ids(&self) -> Vec<ShareableString> {
        vec![ShareableString::new("true"), ShareableString::new("false")]
    }

    /// Returns a vector of descriptions for the choices.
    #[must_use]
    pub fn descriptions(&self) -> Vec<ShareableString> {
        vec![
            self.true_description.clone(),
            self.false_description.clone(),
        ]
    }

    /// Returns a new `BooleanDefinition` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            true_description: store.launder(&self.true_description),
            false_description: store.launder(&self.false_description),
            default_value: store.launder(&self.default_value),
        }
    }

    /// Returns the description of the parameter.
    #[must_use]
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the description.
    #[must_use]
    pub const fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns the default value of the parameter.
    #[must_use]
    pub fn default_value(&self) -> ShareableString {
        self.default_value.clone()
    }

    /// Returns a reference to the default value.
    #[must_use]
    pub const fn default_value_ref(&self) -> &ShareableString {
        &self.default_value
    }
}

impl PartialEq<&BooleanDefinition> for BooleanDefinition {
    fn eq(&self, other: &&BooleanDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<BooleanDefinition> for &BooleanDefinition {
    fn eq(&self, other: &BooleanDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for BooleanDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Boolean - default: \"{}\" [{}]",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value,
            [
                self.true_description.clone(),
                self.false_description.clone()
            ]
            .iter()
            .map(|choice| format!("{} ({})", choice.as_str().to_lowercase(), choice))
            .collect::<Vec<_>>()
            .join(", ")
        )
    }
}
