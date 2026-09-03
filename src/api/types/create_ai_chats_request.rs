pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAiChatsRequest {
    /// The unique identifier of the account to set as context for the AI chat (e.g., "biz_XXXXX").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_account_id: Option<String>,
    /// A list of previously uploaded file attachments to include with the first message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_attachments: Option<Vec<CreateAiChatsRequestMessageAttachmentsItem>>,
    /// The source of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_source: Option<AiChatMessageSourceTypes>,
    /// The text content of the first message to send to the AI agent.
    #[serde(default)]
    pub message_text: String,
    /// The type of suggestion prompt that was clicked, when message_source is 'suggestion'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion_type: Option<String>,
    /// An optional display title for the AI chat thread (e.g., "Help with billing").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CreateAiChatsRequest {
    pub fn builder() -> CreateAiChatsRequestBuilder {
        <CreateAiChatsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAiChatsRequestBuilder {
    current_account_id: Option<String>,
    message_attachments: Option<Vec<CreateAiChatsRequestMessageAttachmentsItem>>,
    message_source: Option<AiChatMessageSourceTypes>,
    message_text: Option<String>,
    suggestion_type: Option<String>,
    title: Option<String>,
}

impl CreateAiChatsRequestBuilder {
    pub fn current_account_id(mut self, value: impl Into<String>) -> Self {
        self.current_account_id = Some(value.into());
        self
    }

    pub fn message_attachments(
        mut self,
        value: Vec<CreateAiChatsRequestMessageAttachmentsItem>,
    ) -> Self {
        self.message_attachments = Some(value);
        self
    }

    pub fn message_source(mut self, value: AiChatMessageSourceTypes) -> Self {
        self.message_source = Some(value);
        self
    }

    pub fn message_text(mut self, value: impl Into<String>) -> Self {
        self.message_text = Some(value.into());
        self
    }

    pub fn suggestion_type(mut self, value: impl Into<String>) -> Self {
        self.suggestion_type = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAiChatsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_text`](CreateAiChatsRequestBuilder::message_text)
    pub fn build(self) -> Result<CreateAiChatsRequest, BuildError> {
        Ok(CreateAiChatsRequest {
            current_account_id: self.current_account_id,
            message_attachments: self.message_attachments,
            message_source: self.message_source,
            message_text: self
                .message_text
                .ok_or_else(|| BuildError::missing_field("message_text"))?,
            suggestion_type: self.suggestion_type,
            title: self.title,
        })
    }
}
