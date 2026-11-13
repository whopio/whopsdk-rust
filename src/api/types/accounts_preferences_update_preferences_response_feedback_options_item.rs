pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesResponseFeedbackOptionsItem {
    /// The category saved as the membership's `cancel_option` when a member picks this option.
    pub category: UpdatePreferencesResponseFeedbackOptionsItemCategory,
    /// Feedback option ID, prefixed `fo_`. Pass it as `feedback_option_id` when canceling a membership, or back in `feedback_options` to keep the option when changing the list.
    #[serde(default)]
    pub id: String,
    /// The cancel reason as members see it. Up to 100 characters.
    #[serde(default)]
    pub label: String,
}

impl UpdatePreferencesResponseFeedbackOptionsItem {
    pub fn builder() -> UpdatePreferencesResponseFeedbackOptionsItemBuilder {
        <UpdatePreferencesResponseFeedbackOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesResponseFeedbackOptionsItemBuilder {
    category: Option<UpdatePreferencesResponseFeedbackOptionsItemCategory>,
    id: Option<String>,
    label: Option<String>,
}

impl UpdatePreferencesResponseFeedbackOptionsItemBuilder {
    pub fn category(mut self, value: UpdatePreferencesResponseFeedbackOptionsItemCategory) -> Self {
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

    /// Consumes the builder and constructs a [`UpdatePreferencesResponseFeedbackOptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](UpdatePreferencesResponseFeedbackOptionsItemBuilder::category)
    /// - [`id`](UpdatePreferencesResponseFeedbackOptionsItemBuilder::id)
    /// - [`label`](UpdatePreferencesResponseFeedbackOptionsItemBuilder::label)
    pub fn build(self) -> Result<UpdatePreferencesResponseFeedbackOptionsItem, BuildError> {
        Ok(UpdatePreferencesResponseFeedbackOptionsItem {
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
