pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChannelMember {
    #[serde(default)]
    pub channel_id: String,
    /// When the stored channel membership was created, as an ISO 8601 timestamp. Null for users whose access does not have a stored membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Stored membership ID, or null for access-derived users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Your inbox state for a private channel. `null` for other users and other channel kinds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_status: Option<ChannelMemberInboxStatus>,
    /// Authorized participant read position. Community positions are visible only to the user themselves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_viewed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_preference: Option<ChannelMemberNotificationPreference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_position: Option<i64>,
    /// Your unread state for the channel. `null` for other users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread: Option<bool>,
    /// When the stored channel membership last changed, as an ISO 8601 timestamp. Null for users whose access does not have a stored membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub user: UserSummary,
    /// User ID
    #[serde(default)]
    pub user_id: String,
}

impl ChannelMember {
    pub fn builder() -> ChannelMemberBuilder {
        <ChannelMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChannelMemberBuilder {
    channel_id: Option<String>,
    created_at: Option<String>,
    id: Option<String>,
    inbox_status: Option<ChannelMemberInboxStatus>,
    last_viewed_at: Option<String>,
    notification_preference: Option<ChannelMemberNotificationPreference>,
    pinned_position: Option<i64>,
    unread: Option<bool>,
    updated_at: Option<String>,
    user: Option<UserSummary>,
    user_id: Option<String>,
}

impl ChannelMemberBuilder {
    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn inbox_status(mut self, value: ChannelMemberInboxStatus) -> Self {
        self.inbox_status = Some(value);
        self
    }

    pub fn last_viewed_at(mut self, value: impl Into<String>) -> Self {
        self.last_viewed_at = Some(value.into());
        self
    }

    pub fn notification_preference(mut self, value: ChannelMemberNotificationPreference) -> Self {
        self.notification_preference = Some(value);
        self
    }

    pub fn pinned_position(mut self, value: i64) -> Self {
        self.pinned_position = Some(value);
        self
    }

    pub fn unread(mut self, value: bool) -> Self {
        self.unread = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn user(mut self, value: UserSummary) -> Self {
        self.user = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChannelMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](ChannelMemberBuilder::channel_id)
    /// - [`user`](ChannelMemberBuilder::user)
    /// - [`user_id`](ChannelMemberBuilder::user_id)
    pub fn build(self) -> Result<ChannelMember, BuildError> {
        Ok(ChannelMember {
            channel_id: self
                .channel_id
                .ok_or_else(|| BuildError::missing_field("channel_id"))?,
            created_at: self.created_at,
            id: self.id,
            inbox_status: self.inbox_status,
            last_viewed_at: self.last_viewed_at,
            notification_preference: self.notification_preference,
            pinned_position: self.pinned_position,
            unread: self.unread,
            updated_at: self.updated_at,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
