pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateFeedbackSubmissionsRequest {
    /// The ID of the account the feedback is about (biz_...), if any. Recorded as context only; it does not require or grant access to that account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The problem or question in plain text, with enough context to act on. Leave out credentials, personal data, and payment details.
    #[serde(default)]
    pub content: String,
    /// What kind of submission this is. Use api_report_feedback for an issue or api_ask_question for an unanswered question. The other values are reserved for Whop's own clients: the hosted MCP server, the Whop CLI, and Whop AI chat.
    pub source: CreateFeedbackSubmissionsRequestSource,
}

impl CreateFeedbackSubmissionsRequest {
    pub fn builder() -> CreateFeedbackSubmissionsRequestBuilder {
        <CreateFeedbackSubmissionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFeedbackSubmissionsRequestBuilder {
    account_id: Option<String>,
    content: Option<String>,
    source: Option<CreateFeedbackSubmissionsRequestSource>,
}

impl CreateFeedbackSubmissionsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn source(mut self, value: CreateFeedbackSubmissionsRequestSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateFeedbackSubmissionsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](CreateFeedbackSubmissionsRequestBuilder::content)
    /// - [`source`](CreateFeedbackSubmissionsRequestBuilder::source)
    pub fn build(self) -> Result<CreateFeedbackSubmissionsRequest, BuildError> {
        Ok(CreateFeedbackSubmissionsRequest {
            account_id: self.account_id,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
