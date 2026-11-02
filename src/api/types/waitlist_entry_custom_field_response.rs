pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WaitlistEntryCustomFieldResponse {
    /// The buyer's answer.
    #[serde(default)]
    pub answer: String,
    /// The answer's ID, prefixed `cfrp_`.
    #[serde(default)]
    pub id: String,
    /// The question shown when the signup was submitted.
    #[serde(default)]
    pub question: String,
}

impl WaitlistEntryCustomFieldResponse {
    pub fn builder() -> WaitlistEntryCustomFieldResponseBuilder {
        <WaitlistEntryCustomFieldResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaitlistEntryCustomFieldResponseBuilder {
    answer: Option<String>,
    id: Option<String>,
    question: Option<String>,
}

impl WaitlistEntryCustomFieldResponseBuilder {
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

    /// Consumes the builder and constructs a [`WaitlistEntryCustomFieldResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`answer`](WaitlistEntryCustomFieldResponseBuilder::answer)
    /// - [`id`](WaitlistEntryCustomFieldResponseBuilder::id)
    /// - [`question`](WaitlistEntryCustomFieldResponseBuilder::question)
    pub fn build(self) -> Result<WaitlistEntryCustomFieldResponse, BuildError> {
        Ok(WaitlistEntryCustomFieldResponse {
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
