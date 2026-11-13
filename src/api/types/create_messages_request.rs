pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMessagesRequest {
    /// Files to attach to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<CreateMessagesRequestAttachmentsItem>>,
    /// Generate previews for URLs in content. Mutually exclusive with link_previews.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_detect_links: Option<bool>,
    /// Public channel ID, or an experience ID for experience chat.
    #[serde(default)]
    pub channel_id: String,
    /// Markdown content. May be empty when the message contains media or a poll.
    #[serde(default)]
    pub content: String,
    /// Up to 10 GIFs to attach. URLs must use HTTPS on static.klipy.com, static1.klipy.com, or static2.klipy.com. Omit to preserve on edit; an empty list clears them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gifs: Option<Vec<CreateMessagesRequestGifsItem>>,
    /// Optional, for clients that show a message before the server confirms it. Use `post_` followed by 22 Base58 characters. Omit to generate an ID. Reuse the same ID and `Idempotency-Key` when retrying a send.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Explicit URLs for the server to preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_previews: Option<Vec<CreateMessagesRequestLinkPreviewsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<CreateMessagesRequestPoll>,
    /// A live message in the same channel to reply to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replying_to_message_id: Option<String>,
}

impl CreateMessagesRequest {
    pub fn builder() -> CreateMessagesRequestBuilder {
        <CreateMessagesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMessagesRequestBuilder {
    attachments: Option<Vec<CreateMessagesRequestAttachmentsItem>>,
    auto_detect_links: Option<bool>,
    channel_id: Option<String>,
    content: Option<String>,
    gifs: Option<Vec<CreateMessagesRequestGifsItem>>,
    id: Option<String>,
    link_previews: Option<Vec<CreateMessagesRequestLinkPreviewsItem>>,
    poll: Option<CreateMessagesRequestPoll>,
    replying_to_message_id: Option<String>,
}

impl CreateMessagesRequestBuilder {
    pub fn attachments(mut self, value: Vec<CreateMessagesRequestAttachmentsItem>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn auto_detect_links(mut self, value: bool) -> Self {
        self.auto_detect_links = Some(value);
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

    pub fn gifs(mut self, value: Vec<CreateMessagesRequestGifsItem>) -> Self {
        self.gifs = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn link_previews(mut self, value: Vec<CreateMessagesRequestLinkPreviewsItem>) -> Self {
        self.link_previews = Some(value);
        self
    }

    pub fn poll(mut self, value: CreateMessagesRequestPoll) -> Self {
        self.poll = Some(value);
        self
    }

    pub fn replying_to_message_id(mut self, value: impl Into<String>) -> Self {
        self.replying_to_message_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMessagesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](CreateMessagesRequestBuilder::channel_id)
    /// - [`content`](CreateMessagesRequestBuilder::content)
    pub fn build(self) -> Result<CreateMessagesRequest, BuildError> {
        Ok(CreateMessagesRequest {
            attachments: self.attachments,
            auto_detect_links: self.auto_detect_links,
            channel_id: self
                .channel_id
                .ok_or_else(|| BuildError::missing_field("channel_id"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            gifs: self.gifs,
            id: self.id,
            link_previews: self.link_previews,
            poll: self.poll,
            replying_to_message_id: self.replying_to_message_id,
        })
    }
}
