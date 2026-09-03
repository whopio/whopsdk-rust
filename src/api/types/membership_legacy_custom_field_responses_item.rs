pub use crate::prelude::*;

/// The response from a custom field on checkout
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipLegacyCustomFieldResponsesItem {
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

impl MembershipLegacyCustomFieldResponsesItem {
    pub fn builder() -> MembershipLegacyCustomFieldResponsesItemBuilder {
        <MembershipLegacyCustomFieldResponsesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipLegacyCustomFieldResponsesItemBuilder {
    answer: Option<String>,
    id: Option<String>,
    question: Option<String>,
}

impl MembershipLegacyCustomFieldResponsesItemBuilder {
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

    /// Consumes the builder and constructs a [`MembershipLegacyCustomFieldResponsesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`answer`](MembershipLegacyCustomFieldResponsesItemBuilder::answer)
    /// - [`id`](MembershipLegacyCustomFieldResponsesItemBuilder::id)
    /// - [`question`](MembershipLegacyCustomFieldResponsesItemBuilder::question)
    pub fn build(self) -> Result<MembershipLegacyCustomFieldResponsesItem, BuildError> {
        Ok(MembershipLegacyCustomFieldResponsesItem {
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
