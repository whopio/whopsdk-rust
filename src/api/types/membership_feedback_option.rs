pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipFeedbackOption {
    /// The option's label when the member picked it. Renaming the option later does not change it.
    #[serde(default)]
    pub label: String,
}

impl MembershipFeedbackOption {
    pub fn builder() -> MembershipFeedbackOptionBuilder {
        <MembershipFeedbackOptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipFeedbackOptionBuilder {
    label: Option<String>,
}

impl MembershipFeedbackOptionBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembershipFeedbackOption`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](MembershipFeedbackOptionBuilder::label)
    pub fn build(self) -> Result<MembershipFeedbackOption, BuildError> {
        Ok(MembershipFeedbackOption {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
