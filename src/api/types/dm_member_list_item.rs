pub use crate::prelude::*;

/// A user's membership record in a messaging channel, including notification preferences and read state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DmMemberListItem {
    /// The unique identifier of the messaging channel this membership belongs to.
    #[serde(default)]
    pub channel_id: String,
    /// The unique identifier for the entity
    #[serde(default)]
    pub id: String,
    /// The timestamp when this member last viewed the channel, as a Unix timestamp in milliseconds. Null if the member has never viewed the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_viewed_at: Option<String>,
    /// The current state of this membership: requested, accepted, hidden, closed, or archived.
    pub status: DmsFeedMemberStatuses,
    /// The unique identifier of the user who holds this channel membership.
    #[serde(default)]
    pub user_id: String,
}

impl DmMemberListItem {
    pub fn builder() -> DmMemberListItemBuilder {
        <DmMemberListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DmMemberListItemBuilder {
    channel_id: Option<String>,
    id: Option<String>,
    last_viewed_at: Option<String>,
    status: Option<DmsFeedMemberStatuses>,
    user_id: Option<String>,
}

impl DmMemberListItemBuilder {
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

    pub fn status(mut self, value: DmsFeedMemberStatuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DmMemberListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](DmMemberListItemBuilder::channel_id)
    /// - [`id`](DmMemberListItemBuilder::id)
    /// - [`status`](DmMemberListItemBuilder::status)
    /// - [`user_id`](DmMemberListItemBuilder::user_id)
    pub fn build(self) -> Result<DmMemberListItem, BuildError> {
        Ok(DmMemberListItem {
            channel_id: self
                .channel_id
                .ok_or_else(|| BuildError::missing_field("channel_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_viewed_at: self.last_viewed_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
