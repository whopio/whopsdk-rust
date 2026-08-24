pub use crate::prelude::*;

/// A message sent within an experience chat, direct message, or group chat.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Message {
    /// The message content formatted as Markdown. Null if the message has no text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The timestamp when this message was originally created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Represents a unique identifier that is Base64 obfuscated. It is often used to refetch an object or as key for a cache. The ID type appears in a JSON response as a String; however, it is not intended to be human-readable. When expected as an input type, any string (such as `"VXNlci0xMA=="`) or integer (such as `4`) input value will be accepted as an ID.
    #[serde(default)]
    pub id: String,
    /// Whether the message content has been edited after it was originally sent.
    #[serde(default)]
    pub is_edited: bool,
    /// Whether this message is pinned to the top of the channel for easy access.
    #[serde(default)]
    pub is_pinned: bool,
    /// A list of user IDs that are explicitly mentioned in this message.
    #[serde(default)]
    pub mentions: Vec<String>,
    /// Whether the message includes an @everyone mention that notifies all channel members.
    #[serde(default)]
    pub mentions_everyone: bool,
    /// The classification of this message: regular, system, or automated.
    pub message_type: DmsPostTypes,
    /// A poll attached to this message. Null if the message does not contain a poll.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<MessagePoll>,
    /// Aggregated reaction counts on this message, filtered to a specific reaction type.
    #[serde(default)]
    pub poll_votes: Vec<MessagePollVotesItem>,
    /// Aggregated reaction counts on this message, filtered to a specific reaction type.
    #[serde(default)]
    pub reaction_counts: Vec<MessageReactionCountsItem>,
    /// The unique identifier of the message this post is replying to. Null if this is not a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replying_to_message_id: Option<String>,
    /// The timestamp when this message was last modified.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The user who authored this message.
    #[serde(default)]
    pub user: MessageUser,
    /// The number of unique views this message has received. Null if view tracking is not enabled for this channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_count: Option<i64>,
}

impl Message {
    pub fn builder() -> MessageBuilder {
        <MessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageBuilder {
    content: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    is_edited: Option<bool>,
    is_pinned: Option<bool>,
    mentions: Option<Vec<String>>,
    mentions_everyone: Option<bool>,
    message_type: Option<DmsPostTypes>,
    poll: Option<MessagePoll>,
    poll_votes: Option<Vec<MessagePollVotesItem>>,
    reaction_counts: Option<Vec<MessageReactionCountsItem>>,
    replying_to_message_id: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    user: Option<MessageUser>,
    view_count: Option<i64>,
}

impl MessageBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
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

    pub fn is_edited(mut self, value: bool) -> Self {
        self.is_edited = Some(value);
        self
    }

    pub fn is_pinned(mut self, value: bool) -> Self {
        self.is_pinned = Some(value);
        self
    }

    pub fn mentions(mut self, value: Vec<String>) -> Self {
        self.mentions = Some(value);
        self
    }

    pub fn mentions_everyone(mut self, value: bool) -> Self {
        self.mentions_everyone = Some(value);
        self
    }

    pub fn message_type(mut self, value: DmsPostTypes) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn poll(mut self, value: MessagePoll) -> Self {
        self.poll = Some(value);
        self
    }

    pub fn poll_votes(mut self, value: Vec<MessagePollVotesItem>) -> Self {
        self.poll_votes = Some(value);
        self
    }

    pub fn reaction_counts(mut self, value: Vec<MessageReactionCountsItem>) -> Self {
        self.reaction_counts = Some(value);
        self
    }

    pub fn replying_to_message_id(mut self, value: impl Into<String>) -> Self {
        self.replying_to_message_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn user(mut self, value: MessageUser) -> Self {
        self.user = Some(value);
        self
    }

    pub fn view_count(mut self, value: i64) -> Self {
        self.view_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Message`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](MessageBuilder::created_at)
    /// - [`id`](MessageBuilder::id)
    /// - [`is_edited`](MessageBuilder::is_edited)
    /// - [`is_pinned`](MessageBuilder::is_pinned)
    /// - [`mentions`](MessageBuilder::mentions)
    /// - [`mentions_everyone`](MessageBuilder::mentions_everyone)
    /// - [`message_type`](MessageBuilder::message_type)
    /// - [`poll_votes`](MessageBuilder::poll_votes)
    /// - [`reaction_counts`](MessageBuilder::reaction_counts)
    /// - [`updated_at`](MessageBuilder::updated_at)
    /// - [`user`](MessageBuilder::user)
    pub fn build(self) -> Result<Message, BuildError> {
        Ok(Message {
            content: self.content,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_edited: self
                .is_edited
                .ok_or_else(|| BuildError::missing_field("is_edited"))?,
            is_pinned: self
                .is_pinned
                .ok_or_else(|| BuildError::missing_field("is_pinned"))?,
            mentions: self
                .mentions
                .ok_or_else(|| BuildError::missing_field("mentions"))?,
            mentions_everyone: self
                .mentions_everyone
                .ok_or_else(|| BuildError::missing_field("mentions_everyone"))?,
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            poll: self.poll,
            poll_votes: self
                .poll_votes
                .ok_or_else(|| BuildError::missing_field("poll_votes"))?,
            reaction_counts: self
                .reaction_counts
                .ok_or_else(|| BuildError::missing_field("reaction_counts"))?,
            replying_to_message_id: self.replying_to_message_id,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
            view_count: self.view_count,
        })
    }
}
