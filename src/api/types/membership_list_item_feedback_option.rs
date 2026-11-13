pub use crate::prelude::*;

/// The account's own cancel reason the member picked, as they saw it. `null` when the member picked a default category or gave no reason.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipListItemFeedbackOption {
    /// The option's label when the member picked it. Renaming the option later does not change it.
    #[serde(default)]
    pub label: String,
}

impl MembershipListItemFeedbackOption {
    pub fn builder() -> MembershipListItemFeedbackOptionBuilder {
        <MembershipListItemFeedbackOptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipListItemFeedbackOptionBuilder {
    label: Option<String>,
}

impl MembershipListItemFeedbackOptionBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembershipListItemFeedbackOption`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](MembershipListItemFeedbackOptionBuilder::label)
    pub fn build(self) -> Result<MembershipListItemFeedbackOption, BuildError> {
        Ok(MembershipListItemFeedbackOption {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
