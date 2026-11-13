pub use crate::prelude::*;

/// The account's own cancel reason the member picked, as they saw it. `null` when the member picked a default category or gave no reason.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipLegacyFeedbackOption {
    /// The option's label when the member picked it. Renaming the option later does not change it.
    #[serde(default)]
    pub label: String,
}

impl MembershipLegacyFeedbackOption {
    pub fn builder() -> MembershipLegacyFeedbackOptionBuilder {
        <MembershipLegacyFeedbackOptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipLegacyFeedbackOptionBuilder {
    label: Option<String>,
}

impl MembershipLegacyFeedbackOptionBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembershipLegacyFeedbackOption`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](MembershipLegacyFeedbackOptionBuilder::label)
    pub fn build(self) -> Result<MembershipLegacyFeedbackOption, BuildError> {
        Ok(MembershipLegacyFeedbackOption {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
