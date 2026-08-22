pub use crate::prelude::*;

/// An AI-powered chat conversation belonging to a user, with optional scheduled automation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AiChat {
    /// The total number of tokens consumed across all messages in this conversation.
    #[serde(default)]
    pub blended_token_usage: String,
    /// The datetime the ai chat was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The unique identifier for the ai chat.
    #[serde(default)]
    pub id: String,
    /// The timestamp of the most recent message in this conversation. Null if no messages have been sent yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_message_at: Option<DateTime<FixedOffset>>,
    /// The total number of messages exchanged in this conversation.
    #[serde(default)]
    pub message_count: i64,
    /// The notification preference for this AI chat. `all` delivers AI chat notifications and badges, while `none` mutes them.
    pub notification_preference: AiChatNotificationPreferences,
    /// A short descriptive title for this AI chat conversation. Null if no title has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The datetime the ai chat was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The user who owns this AI chat conversation.
    #[serde(default)]
    pub user: AiChatUser,
}

impl AiChat {
    pub fn builder() -> AiChatBuilder {
        <AiChatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AiChatBuilder {
    blended_token_usage: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    last_message_at: Option<DateTime<FixedOffset>>,
    message_count: Option<i64>,
    notification_preference: Option<AiChatNotificationPreferences>,
    title: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    user: Option<AiChatUser>,
}

impl AiChatBuilder {
    pub fn blended_token_usage(mut self, value: impl Into<String>) -> Self {
        self.blended_token_usage = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_message_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_message_at = Some(value);
        self
    }

    pub fn message_count(mut self, value: i64) -> Self {
        self.message_count = Some(value);
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

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn user(mut self, value: AiChatUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AiChat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`blended_token_usage`](AiChatBuilder::blended_token_usage)
    /// - [`created_at`](AiChatBuilder::created_at)
    /// - [`id`](AiChatBuilder::id)
    /// - [`message_count`](AiChatBuilder::message_count)
    /// - [`notification_preference`](AiChatBuilder::notification_preference)
    /// - [`updated_at`](AiChatBuilder::updated_at)
    /// - [`user`](AiChatBuilder::user)
    pub fn build(self) -> Result<AiChat, BuildError> {
        Ok(AiChat {
            blended_token_usage: self
                .blended_token_usage
                .ok_or_else(|| BuildError::missing_field("blended_token_usage"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_message_at: self.last_message_at,
            message_count: self
                .message_count
                .ok_or_else(|| BuildError::missing_field("message_count"))?,
            notification_preference: self
                .notification_preference
                .ok_or_else(|| BuildError::missing_field("notification_preference"))?,
            title: self.title,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
