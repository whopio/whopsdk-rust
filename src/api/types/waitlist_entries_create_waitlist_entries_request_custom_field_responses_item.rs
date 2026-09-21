pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWaitlistEntriesRequestCustomFieldResponsesItem {
    /// The buyer's answer.
    #[serde(default)]
    pub answer: String,
    /// The checkout question being answered, prefixed `field_`.
    #[serde(default)]
    pub custom_field_id: String,
}

impl CreateWaitlistEntriesRequestCustomFieldResponsesItem {
    pub fn builder() -> CreateWaitlistEntriesRequestCustomFieldResponsesItemBuilder {
        <CreateWaitlistEntriesRequestCustomFieldResponsesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWaitlistEntriesRequestCustomFieldResponsesItemBuilder {
    answer: Option<String>,
    custom_field_id: Option<String>,
}

impl CreateWaitlistEntriesRequestCustomFieldResponsesItemBuilder {
    pub fn answer(mut self, value: impl Into<String>) -> Self {
        self.answer = Some(value.into());
        self
    }

    pub fn custom_field_id(mut self, value: impl Into<String>) -> Self {
        self.custom_field_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateWaitlistEntriesRequestCustomFieldResponsesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`answer`](CreateWaitlistEntriesRequestCustomFieldResponsesItemBuilder::answer)
    /// - [`custom_field_id`](CreateWaitlistEntriesRequestCustomFieldResponsesItemBuilder::custom_field_id)
    pub fn build(self) -> Result<CreateWaitlistEntriesRequestCustomFieldResponsesItem, BuildError> {
        Ok(CreateWaitlistEntriesRequestCustomFieldResponsesItem {
            answer: self
                .answer
                .ok_or_else(|| BuildError::missing_field("answer"))?,
            custom_field_id: self
                .custom_field_id
                .ok_or_else(|| BuildError::missing_field("custom_field_id"))?,
        })
    }
}
