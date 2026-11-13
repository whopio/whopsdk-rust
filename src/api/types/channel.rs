pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Channel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Creation time as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// When the channel was deleted or removed from your channel list, as an ISO 8601 timestamp. Null for a live channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_id: Option<String>,
    /// The channel icon. Community channels show the experience logo, falling back to the app icon. Support channels show the account logo. Group private channels show their own icon. Null for one-to-one private channels or when no image is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<File>,
    /// Existing public channel ID, preserved across REST and chat clients.
    #[serde(default)]
    pub id: String,
    pub kind: ChannelKind,
    /// When the channel last recorded message activity, as an ISO 8601 timestamp. Null when no message activity is recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership: Option<ChannelMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<ChannelModeration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether app notifications are on. Webhooks fire either way.
    #[serde(default)]
    pub notifications_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support: Option<ChannelSupportState>,
    /// Last channel change as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl Channel {
    pub fn builder() -> ChannelBuilder {
        <ChannelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChannelBuilder {
    account_id: Option<String>,
    created_at: Option<String>,
    deleted_at: Option<String>,
    experience_id: Option<String>,
    icon: Option<File>,
    id: Option<String>,
    kind: Option<ChannelKind>,
    last_message_at: Option<String>,
    last_message_id: Option<String>,
    membership: Option<ChannelMember>,
    moderation: Option<ChannelModeration>,
    name: Option<String>,
    notifications_enabled: Option<bool>,
    support: Option<ChannelSupportState>,
    updated_at: Option<String>,
}

impl ChannelBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn deleted_at(mut self, value: impl Into<String>) -> Self {
        self.deleted_at = Some(value.into());
        self
    }

    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    pub fn icon(mut self, value: File) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: ChannelKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn last_message_at(mut self, value: impl Into<String>) -> Self {
        self.last_message_at = Some(value.into());
        self
    }

    pub fn last_message_id(mut self, value: impl Into<String>) -> Self {
        self.last_message_id = Some(value.into());
        self
    }

    pub fn membership(mut self, value: ChannelMember) -> Self {
        self.membership = Some(value);
        self
    }

    pub fn moderation(mut self, value: ChannelModeration) -> Self {
        self.moderation = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn notifications_enabled(mut self, value: bool) -> Self {
        self.notifications_enabled = Some(value);
        self
    }

    pub fn support(mut self, value: ChannelSupportState) -> Self {
        self.support = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Channel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ChannelBuilder::created_at)
    /// - [`id`](ChannelBuilder::id)
    /// - [`kind`](ChannelBuilder::kind)
    /// - [`notifications_enabled`](ChannelBuilder::notifications_enabled)
    /// - [`updated_at`](ChannelBuilder::updated_at)
    pub fn build(self) -> Result<Channel, BuildError> {
        Ok(Channel {
            account_id: self.account_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            experience_id: self.experience_id,
            icon: self.icon,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            last_message_at: self.last_message_at,
            last_message_id: self.last_message_id,
            membership: self.membership,
            moderation: self.moderation,
            name: self.name,
            notifications_enabled: self
                .notifications_enabled
                .ok_or_else(|| BuildError::missing_field("notifications_enabled"))?,
            support: self.support,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
