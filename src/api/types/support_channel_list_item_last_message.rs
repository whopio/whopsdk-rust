pub use crate::prelude::*;

/// The most recent message in this channel. Null if no messages have been sent, or when listing support channels without `include_last_message`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SupportChannelListItemLastMessage {
    /// A list of all attachments on this message, including files and media.
    #[serde(default)]
    pub attachments: Vec<SupportChannelListItemLastMessageAttachmentsItem>,
    /// The message content formatted as Markdown. Null if the message has no text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The timestamp when this message was originally created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// A list of GIF animations attached to this message.
    #[serde(default)]
    pub gifs: Vec<SupportChannelListItemLastMessageGifsItem>,
    /// Represents a unique identifier that is Base64 obfuscated. It is often used to refetch an object or as key for a cache. The ID type appears in a JSON response as a String; however, it is not intended to be human-readable. When expected as an input type, any string (such as `"VXNlci0xMA=="`) or integer (such as `4`) input value will be accepted as an ID.
    #[serde(default)]
    pub id: String,
    /// A poll attached to this message. Null if the message does not contain a poll.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<SupportChannelListItemLastMessagePoll>,
    /// The unique identifier of the user who authored this message.
    #[serde(default)]
    pub user_id: String,
}

impl SupportChannelListItemLastMessage {
    pub fn builder() -> SupportChannelListItemLastMessageBuilder {
        <SupportChannelListItemLastMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SupportChannelListItemLastMessageBuilder {
    attachments: Option<Vec<SupportChannelListItemLastMessageAttachmentsItem>>,
    content: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    gifs: Option<Vec<SupportChannelListItemLastMessageGifsItem>>,
    id: Option<String>,
    poll: Option<SupportChannelListItemLastMessagePoll>,
    user_id: Option<String>,
}

impl SupportChannelListItemLastMessageBuilder {
    pub fn attachments(
        mut self,
        value: Vec<SupportChannelListItemLastMessageAttachmentsItem>,
    ) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn gifs(mut self, value: Vec<SupportChannelListItemLastMessageGifsItem>) -> Self {
        self.gifs = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn poll(mut self, value: SupportChannelListItemLastMessagePoll) -> Self {
        self.poll = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SupportChannelListItemLastMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attachments`](SupportChannelListItemLastMessageBuilder::attachments)
    /// - [`created_at`](SupportChannelListItemLastMessageBuilder::created_at)
    /// - [`gifs`](SupportChannelListItemLastMessageBuilder::gifs)
    /// - [`id`](SupportChannelListItemLastMessageBuilder::id)
    /// - [`user_id`](SupportChannelListItemLastMessageBuilder::user_id)
    pub fn build(self) -> Result<SupportChannelListItemLastMessage, BuildError> {
        Ok(SupportChannelListItemLastMessage {
            attachments: self
                .attachments
                .ok_or_else(|| BuildError::missing_field("attachments"))?,
            content: self.content,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            gifs: self.gifs.ok_or_else(|| BuildError::missing_field("gifs"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            poll: self.poll,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
