pub use crate::prelude::*;

/// A messaging channel that can be a one-on-one DM, group chat, company support conversation, or platform-level direct message.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DmChannelListItem {
    /// The time the entity was created (in milliseconds since Unix epoch)
    #[serde(default)]
    pub created_at: String,
    /// The unique identifier for the entity
    #[serde(default)]
    pub id: String,
    /// The timestamp when the most recent message was sent in this channel. Null if no messages have been sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_message_at: Option<DateTime<FixedOffset>>,
    /// A custom display name assigned to this channel by the user. Null if no custom name has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DmChannelListItem {
    pub fn builder() -> DmChannelListItemBuilder {
        <DmChannelListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DmChannelListItemBuilder {
    created_at: Option<String>,
    id: Option<String>,
    last_message_at: Option<DateTime<FixedOffset>>,
    name: Option<String>,
}

impl DmChannelListItemBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
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

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DmChannelListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](DmChannelListItemBuilder::created_at)
    /// - [`id`](DmChannelListItemBuilder::id)
    pub fn build(self) -> Result<DmChannelListItem, BuildError> {
        Ok(DmChannelListItem {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_message_at: self.last_message_at,
            name: self.name,
        })
    }
}
