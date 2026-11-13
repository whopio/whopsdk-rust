pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesRequestFeedbackOptionsItem {
    /// The category saved as the membership's `cancel_option` when a member picks this option, so reports keep grouping cancellations.
    pub category: UpdatePreferencesRequestFeedbackOptionsItemCategory,
    /// The ID of an option to keep. Omit it to add a new option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The cancel reason as members see it.
    #[serde(default)]
    pub label: String,
}

impl UpdatePreferencesRequestFeedbackOptionsItem {
    pub fn builder() -> UpdatePreferencesRequestFeedbackOptionsItemBuilder {
        <UpdatePreferencesRequestFeedbackOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesRequestFeedbackOptionsItemBuilder {
    category: Option<UpdatePreferencesRequestFeedbackOptionsItemCategory>,
    id: Option<String>,
    label: Option<String>,
}

impl UpdatePreferencesRequestFeedbackOptionsItemBuilder {
    pub fn category(mut self, value: UpdatePreferencesRequestFeedbackOptionsItemCategory) -> Self {
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

    /// Consumes the builder and constructs a [`UpdatePreferencesRequestFeedbackOptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](UpdatePreferencesRequestFeedbackOptionsItemBuilder::category)
    /// - [`label`](UpdatePreferencesRequestFeedbackOptionsItemBuilder::label)
    pub fn build(self) -> Result<UpdatePreferencesRequestFeedbackOptionsItem, BuildError> {
        Ok(UpdatePreferencesRequestFeedbackOptionsItem {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            id: self.id,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
