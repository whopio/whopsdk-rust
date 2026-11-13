pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    #[serde(default)]
    pub attachments: Vec<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<UserSummary>,
    /// `true` if the author is a team member of the account that owns this community channel. `false` for partners, private and support channels, and deleted messages.
    #[serde(default)]
    pub author_is_team_member: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_user_id: Option<String>,
    #[serde(default)]
    pub channel_id: String,
    /// Markdown content. `null` for a deleted message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Creation time as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_author: Option<MessageCustomAuthor>,
    /// Deletion time, or null for a live message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[serde(default)]
    pub edited: bool,
    #[serde(default)]
    pub embeds: Vec<Embed>,
    #[serde(default)]
    pub gifs: Vec<Gif>,
    /// Message ID, prefixed `post_`. Matches the `id` sent on create, when one was sent.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub link_previews: Vec<MessageLinkPreview>,
    #[serde(default)]
    pub mentioned_user_ids: Vec<String>,
    pub message_type: ChatMessageMessageType,
    #[serde(default)]
    pub pinned: bool,
    /// Unformatted message text. null when the text is absent or the message is deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<ChatMessagePoll>,
    #[serde(default)]
    pub reaction_counts: Vec<MessageReactionCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replying_to_message_id: Option<String>,
    /// User ID of the replied-to message's author, prefixed `user_`. `null` without a reply in this channel, or when either message is deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replying_to_user_id: Option<String>,
    /// Last message change as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// Experience view count. `null` for DM-backed channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_count: Option<i64>,
}

impl ChatMessage {
    pub fn builder() -> ChatMessageBuilder {
        <ChatMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatMessageBuilder {
    attachments: Option<Vec<File>>,
    author: Option<UserSummary>,
    author_is_team_member: Option<bool>,
    author_user_id: Option<String>,
    channel_id: Option<String>,
    content: Option<String>,
    created_at: Option<String>,
    custom_author: Option<MessageCustomAuthor>,
    deleted_at: Option<String>,
    edited: Option<bool>,
    embeds: Option<Vec<Embed>>,
    gifs: Option<Vec<Gif>>,
    id: Option<String>,
    link_previews: Option<Vec<MessageLinkPreview>>,
    mentioned_user_ids: Option<Vec<String>>,
    message_type: Option<ChatMessageMessageType>,
    pinned: Option<bool>,
    plain_content: Option<String>,
    poll: Option<ChatMessagePoll>,
    reaction_counts: Option<Vec<MessageReactionCount>>,
    replying_to_message_id: Option<String>,
    replying_to_user_id: Option<String>,
    updated_at: Option<String>,
    view_count: Option<i64>,
}

impl ChatMessageBuilder {
    pub fn attachments(mut self, value: Vec<File>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn author(mut self, value: UserSummary) -> Self {
        self.author = Some(value);
        self
    }

    pub fn author_is_team_member(mut self, value: bool) -> Self {
        self.author_is_team_member = Some(value);
        self
    }

    pub fn author_user_id(mut self, value: impl Into<String>) -> Self {
        self.author_user_id = Some(value.into());
        self
    }

    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn custom_author(mut self, value: MessageCustomAuthor) -> Self {
        self.custom_author = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: impl Into<String>) -> Self {
        self.deleted_at = Some(value.into());
        self
    }

    pub fn edited(mut self, value: bool) -> Self {
        self.edited = Some(value);
        self
    }

    pub fn embeds(mut self, value: Vec<Embed>) -> Self {
        self.embeds = Some(value);
        self
    }

    pub fn gifs(mut self, value: Vec<Gif>) -> Self {
        self.gifs = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn link_previews(mut self, value: Vec<MessageLinkPreview>) -> Self {
        self.link_previews = Some(value);
        self
    }

    pub fn mentioned_user_ids(mut self, value: Vec<String>) -> Self {
        self.mentioned_user_ids = Some(value);
        self
    }

    pub fn message_type(mut self, value: ChatMessageMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn pinned(mut self, value: bool) -> Self {
        self.pinned = Some(value);
        self
    }

    pub fn plain_content(mut self, value: impl Into<String>) -> Self {
        self.plain_content = Some(value.into());
        self
    }

    pub fn poll(mut self, value: ChatMessagePoll) -> Self {
        self.poll = Some(value);
        self
    }

    pub fn reaction_counts(mut self, value: Vec<MessageReactionCount>) -> Self {
        self.reaction_counts = Some(value);
        self
    }

    pub fn replying_to_message_id(mut self, value: impl Into<String>) -> Self {
        self.replying_to_message_id = Some(value.into());
        self
    }

    pub fn replying_to_user_id(mut self, value: impl Into<String>) -> Self {
        self.replying_to_user_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn view_count(mut self, value: i64) -> Self {
        self.view_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attachments`](ChatMessageBuilder::attachments)
    /// - [`author_is_team_member`](ChatMessageBuilder::author_is_team_member)
    /// - [`channel_id`](ChatMessageBuilder::channel_id)
    /// - [`created_at`](ChatMessageBuilder::created_at)
    /// - [`edited`](ChatMessageBuilder::edited)
    /// - [`embeds`](ChatMessageBuilder::embeds)
    /// - [`gifs`](ChatMessageBuilder::gifs)
    /// - [`id`](ChatMessageBuilder::id)
    /// - [`link_previews`](ChatMessageBuilder::link_previews)
    /// - [`mentioned_user_ids`](ChatMessageBuilder::mentioned_user_ids)
    /// - [`message_type`](ChatMessageBuilder::message_type)
    /// - [`pinned`](ChatMessageBuilder::pinned)
    /// - [`reaction_counts`](ChatMessageBuilder::reaction_counts)
    /// - [`updated_at`](ChatMessageBuilder::updated_at)
    pub fn build(self) -> Result<ChatMessage, BuildError> {
        Ok(ChatMessage {
            attachments: self
                .attachments
                .ok_or_else(|| BuildError::missing_field("attachments"))?,
            author: self.author,
            author_is_team_member: self
                .author_is_team_member
                .ok_or_else(|| BuildError::missing_field("author_is_team_member"))?,
            author_user_id: self.author_user_id,
            channel_id: self
                .channel_id
                .ok_or_else(|| BuildError::missing_field("channel_id"))?,
            content: self.content,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            custom_author: self.custom_author,
            deleted_at: self.deleted_at,
            edited: self
                .edited
                .ok_or_else(|| BuildError::missing_field("edited"))?,
            embeds: self
                .embeds
                .ok_or_else(|| BuildError::missing_field("embeds"))?,
            gifs: self.gifs.ok_or_else(|| BuildError::missing_field("gifs"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            link_previews: self
                .link_previews
                .ok_or_else(|| BuildError::missing_field("link_previews"))?,
            mentioned_user_ids: self
                .mentioned_user_ids
                .ok_or_else(|| BuildError::missing_field("mentioned_user_ids"))?,
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            pinned: self
                .pinned
                .ok_or_else(|| BuildError::missing_field("pinned"))?,
            plain_content: self.plain_content,
            poll: self.poll,
            reaction_counts: self
                .reaction_counts
                .ok_or_else(|| BuildError::missing_field("reaction_counts"))?,
            replying_to_message_id: self.replying_to_message_id,
            replying_to_user_id: self.replying_to_user_id,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            view_count: self.view_count,
        })
    }
}
