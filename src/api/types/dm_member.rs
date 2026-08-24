pub use crate::prelude::*;

/// A user's membership record in a messaging channel, including notification preferences and read state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DmMember {
    /// The unique identifier of the messaging channel this membership belongs to.
    #[serde(default)]
    pub channel_id: String,
    /// The unique identifier for the entity
    #[serde(default)]
    pub id: String,
    /// The timestamp when this member last viewed the channel, as a Unix timestamp in milliseconds. Null if the member has never viewed the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_viewed_at: Option<String>,
    /// The notification level for this channel: all, mentions, or none.
    pub notification_preference: DmsFeedMemberNotificationPreferences,
    /// The current state of this membership: requested, accepted, hidden, closed, or archived.
    pub status: DmsFeedMemberStatuses,
    /// The unique identifier of the user who holds this channel membership.
    #[serde(default)]
    pub user_id: String,
}

impl DmMember {
    pub fn builder() -> DmMemberBuilder {
        <DmMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DmMemberBuilder {
    channel_id: Option<String>,
    id: Option<String>,
    last_viewed_at: Option<String>,
    notification_preference: Option<DmsFeedMemberNotificationPreferences>,
    status: Option<DmsFeedMemberStatuses>,
    user_id: Option<String>,
}

impl DmMemberBuilder {
    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_viewed_at(mut self, value: impl Into<String>) -> Self {
        self.last_viewed_at = Some(value.into());
        self
    }

    pub fn notification_preference(mut self, value: DmsFeedMemberNotificationPreferences) -> Self {
        self.notification_preference = Some(value);
        self
    }

    pub fn status(mut self, value: DmsFeedMemberStatuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DmMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](DmMemberBuilder::channel_id)
    /// - [`id`](DmMemberBuilder::id)
    /// - [`notification_preference`](DmMemberBuilder::notification_preference)
    /// - [`status`](DmMemberBuilder::status)
    /// - [`user_id`](DmMemberBuilder::user_id)
    pub fn build(self) -> Result<DmMember, BuildError> {
        Ok(DmMember {
            channel_id: self
                .channel_id
                .ok_or_else(|| BuildError::missing_field("channel_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_viewed_at: self.last_viewed_at,
            notification_preference: self
                .notification_preference
                .ok_or_else(|| BuildError::missing_field("notification_preference"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
