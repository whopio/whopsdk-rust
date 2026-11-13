pub use crate::prelude::*;

/// A messaging channel that can be a one-on-one DM, group chat, company support conversation, or platform-level direct message.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SupportChannel {
    /// The unique identifier of the account associated with this channel. Null if this is not a support or account-scoped conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The unique identifier of the account associated with this channel. Null if this is not a support or account-scoped conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// A custom display name assigned to this channel by the user. Null if no custom name has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// The customer who initiated this support conversation. Null if this is not a support chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_user: Option<SupportChannelCustomerUser>,
    /// The unique identifier for the entity
    #[serde(default)]
    pub id: String,
    /// The most recent message in this channel. Null if no messages have been sent, or when listing support channels without `include_last_message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message: Option<SupportChannelLastMessage>,
    /// The timestamp when the most recent message was sent in this channel. Null if no messages have been sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_message_at: Option<DateTime<FixedOffset>>,
    /// Whether the customer sent the latest message and the linked support ticket is unresolved. Null if not a support chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_reply: Option<bool>,
    /// Whether the authenticated user has joined this channel as a participant. Null when the caller is not a user, or when listing support channels without `include_participated`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participated: Option<bool>,
    /// The timestamp when the linked support ticket was marked as resolved. Null if unresolved or not a support chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub resolved_at: Option<DateTime<FixedOffset>>,
    /// Whether this channel has a message from someone else that the authenticated user has not viewed yet. Null when the caller is not a user, or when listing support channels without `include_last_message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread: Option<bool>,
}

impl SupportChannel {
    pub fn builder() -> SupportChannelBuilder {
        <SupportChannelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SupportChannelBuilder {
    account_id: Option<String>,
    company_id: Option<String>,
    custom_name: Option<String>,
    customer_user: Option<SupportChannelCustomerUser>,
    id: Option<String>,
    last_message: Option<SupportChannelLastMessage>,
    last_message_at: Option<DateTime<FixedOffset>>,
    needs_reply: Option<bool>,
    participated: Option<bool>,
    resolved_at: Option<DateTime<FixedOffset>>,
    unread: Option<bool>,
}

impl SupportChannelBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn custom_name(mut self, value: impl Into<String>) -> Self {
        self.custom_name = Some(value.into());
        self
    }

    pub fn customer_user(mut self, value: SupportChannelCustomerUser) -> Self {
        self.customer_user = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_message(mut self, value: SupportChannelLastMessage) -> Self {
        self.last_message = Some(value);
        self
    }

    pub fn last_message_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_message_at = Some(value);
        self
    }

    pub fn needs_reply(mut self, value: bool) -> Self {
        self.needs_reply = Some(value);
        self
    }

    pub fn participated(mut self, value: bool) -> Self {
        self.participated = Some(value);
        self
    }

    pub fn resolved_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.resolved_at = Some(value);
        self
    }

    pub fn unread(mut self, value: bool) -> Self {
        self.unread = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SupportChannel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SupportChannelBuilder::id)
    pub fn build(self) -> Result<SupportChannel, BuildError> {
        Ok(SupportChannel {
            account_id: self.account_id,
            company_id: self.company_id,
            custom_name: self.custom_name,
            customer_user: self.customer_user,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_message: self.last_message,
            last_message_at: self.last_message_at,
            needs_reply: self.needs_reply,
            participated: self.participated,
            resolved_at: self.resolved_at,
            unread: self.unread,
        })
    }
}
