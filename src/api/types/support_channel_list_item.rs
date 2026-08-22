pub use crate::prelude::*;

/// A messaging channel that can be a one-on-one DM, group chat, company support conversation, or platform-level direct message.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SupportChannelListItem {
    /// The unique identifier of the company associated with this channel. Null if this is not a support or company-scoped conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// A custom display name assigned to this channel by the user. Null if no custom name has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// The customer who initiated this support conversation. Null if this is not a support chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_user: Option<SupportChannelListItemCustomerUser>,
    /// The unique identifier for the entity
    #[serde(default)]
    pub id: String,
    /// The timestamp when the most recent message was sent in this channel. Null if no messages have been sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_message_at: Option<DateTime<FixedOffset>>,
    /// The timestamp when the linked support ticket was marked as resolved. Null if unresolved or not a support chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub resolved_at: Option<DateTime<FixedOffset>>,
}

impl SupportChannelListItem {
    pub fn builder() -> SupportChannelListItemBuilder {
        <SupportChannelListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SupportChannelListItemBuilder {
    company_id: Option<String>,
    custom_name: Option<String>,
    customer_user: Option<SupportChannelListItemCustomerUser>,
    id: Option<String>,
    last_message_at: Option<DateTime<FixedOffset>>,
    resolved_at: Option<DateTime<FixedOffset>>,
}

impl SupportChannelListItemBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn custom_name(mut self, value: impl Into<String>) -> Self {
        self.custom_name = Some(value.into());
        self
    }

    pub fn customer_user(mut self, value: SupportChannelListItemCustomerUser) -> Self {
        self.customer_user = Some(value);
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

    pub fn resolved_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.resolved_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SupportChannelListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SupportChannelListItemBuilder::id)
    pub fn build(self) -> Result<SupportChannelListItem, BuildError> {
        Ok(SupportChannelListItem {
            company_id: self.company_id,
            custom_name: self.custom_name,
            customer_user: self.customer_user,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_message_at: self.last_message_at,
            resolved_at: self.resolved_at,
        })
    }
}
