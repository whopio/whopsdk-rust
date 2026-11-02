pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAiChatsRequest {
    /// The unique identifier of the account to set as context for the AI chat (e.g., "biz_XXXXX").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_account_id: Option<String>,
    /// The notification preference for the AI chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_preference: Option<AiChatNotificationPreferences>,
    /// The new display title for the AI chat thread (e.g., "Help with billing").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl UpdateAiChatsRequest {
    pub fn builder() -> UpdateAiChatsRequestBuilder {
        <UpdateAiChatsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAiChatsRequestBuilder {
    current_account_id: Option<String>,
    notification_preference: Option<AiChatNotificationPreferences>,
    title: Option<String>,
}

impl UpdateAiChatsRequestBuilder {
    pub fn current_account_id(mut self, value: impl Into<String>) -> Self {
        self.current_account_id = Some(value.into());
        self
    }

    pub fn notification_preference(mut self, value: AiChatNotificationPreferences) -> Self {
        self.notification_preference = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAiChatsRequest`].
    pub fn build(self) -> Result<UpdateAiChatsRequest, BuildError> {
        Ok(UpdateAiChatsRequest {
            current_account_id: self.current_account_id,
            notification_preference: self.notification_preference,
            title: self.title,
        })
    }
}
