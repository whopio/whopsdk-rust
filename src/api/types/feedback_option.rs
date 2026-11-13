pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FeedbackOption {
    /// The category saved as the membership's `cancel_option` when a member picks this option.
    pub category: FeedbackOptionCategory,
    /// Feedback option ID, prefixed `fo_`. Pass it as `feedback_option_id` when canceling a membership, or back in `feedback_options` to keep the option when changing the list.
    #[serde(default)]
    pub id: String,
    /// The cancel reason as members see it. Up to 100 characters.
    #[serde(default)]
    pub label: String,
}

impl FeedbackOption {
    pub fn builder() -> FeedbackOptionBuilder {
        <FeedbackOptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedbackOptionBuilder {
    category: Option<FeedbackOptionCategory>,
    id: Option<String>,
    label: Option<String>,
}

impl FeedbackOptionBuilder {
    pub fn category(mut self, value: FeedbackOptionCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FeedbackOption`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](FeedbackOptionBuilder::category)
    /// - [`id`](FeedbackOptionBuilder::id)
    /// - [`label`](FeedbackOptionBuilder::label)
    pub fn build(self) -> Result<FeedbackOption, BuildError> {
        Ok(FeedbackOption {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
