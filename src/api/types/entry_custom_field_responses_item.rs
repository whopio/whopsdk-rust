pub use crate::prelude::*;

/// The response from a custom field on checkout
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntryCustomFieldResponsesItem {
    /// The response a user gave to the specific question or field.
    #[serde(default)]
    pub answer: String,
    /// The unique identifier for the custom field response.
    #[serde(default)]
    pub id: String,
    /// The question asked by the custom field
    #[serde(default)]
    pub question: String,
}

impl EntryCustomFieldResponsesItem {
    pub fn builder() -> EntryCustomFieldResponsesItemBuilder {
        <EntryCustomFieldResponsesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntryCustomFieldResponsesItemBuilder {
    answer: Option<String>,
    id: Option<String>,
    question: Option<String>,
}

impl EntryCustomFieldResponsesItemBuilder {
    pub fn answer(mut self, value: impl Into<String>) -> Self {
        self.answer = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn question(mut self, value: impl Into<String>) -> Self {
        self.question = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EntryCustomFieldResponsesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`answer`](EntryCustomFieldResponsesItemBuilder::answer)
    /// - [`id`](EntryCustomFieldResponsesItemBuilder::id)
    /// - [`question`](EntryCustomFieldResponsesItemBuilder::question)
    pub fn build(self) -> Result<EntryCustomFieldResponsesItem, BuildError> {
        Ok(EntryCustomFieldResponsesItem {
            answer: self
                .answer
                .ok_or_else(|| BuildError::missing_field("answer"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            question: self
                .question
                .ok_or_else(|| BuildError::missing_field("question"))?,
        })
    }
}
