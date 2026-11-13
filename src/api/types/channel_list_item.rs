pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelListItem {
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
    pub kind: ChannelListItemKind,
    /// Latest visible message, or null. Present when include_last_message=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message: Option<ChatMessage>,
    /// When the channel last recorded message activity, as an ISO 8601 timestamp. Null when no message activity is recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<ChannelMember>>,
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
    /// Latest change to the channel or your membership, as an ISO 8601 timestamp. Use this value as updated_after for synchronization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronized_at: Option<String>,
    /// Last channel change as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl ChannelListItem {
    pub fn builder() -> ChannelListItemBuilder {
        <ChannelListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChannelListItemBuilder {
    account_id: Option<String>,
    created_at: Option<String>,
    deleted_at: Option<String>,
    experience_id: Option<String>,
    icon: Option<File>,
    id: Option<String>,
    kind: Option<ChannelListItemKind>,
    last_message: Option<ChatMessage>,
    last_message_at: Option<String>,
    last_message_id: Option<String>,
    members: Option<Vec<ChannelMember>>,
    membership: Option<ChannelMember>,
    moderation: Option<ChannelModeration>,
    name: Option<String>,
    notifications_enabled: Option<bool>,
    support: Option<ChannelSupportState>,
    synchronized_at: Option<String>,
    updated_at: Option<String>,
}

impl ChannelListItemBuilder {
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

    pub fn kind(mut self, value: ChannelListItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn last_message(mut self, value: ChatMessage) -> Self {
        self.last_message = Some(value);
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

    pub fn members(mut self, value: Vec<ChannelMember>) -> Self {
        self.members = Some(value);
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

    pub fn synchronized_at(mut self, value: impl Into<String>) -> Self {
        self.synchronized_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChannelListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ChannelListItemBuilder::created_at)
    /// - [`id`](ChannelListItemBuilder::id)
    /// - [`kind`](ChannelListItemBuilder::kind)
    /// - [`notifications_enabled`](ChannelListItemBuilder::notifications_enabled)
    /// - [`updated_at`](ChannelListItemBuilder::updated_at)
    pub fn build(self) -> Result<ChannelListItem, BuildError> {
        Ok(ChannelListItem {
            account_id: self.account_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            experience_id: self.experience_id,
            icon: self.icon,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            last_message: self.last_message,
            last_message_at: self.last_message_at,
            last_message_id: self.last_message_id,
            members: self.members,
            membership: self.membership,
            moderation: self.moderation,
            name: self.name,
            notifications_enabled: self
                .notifications_enabled
                .ok_or_else(|| BuildError::missing_field("notifications_enabled"))?,
            support: self.support,
            synchronized_at: self.synchronized_at,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
