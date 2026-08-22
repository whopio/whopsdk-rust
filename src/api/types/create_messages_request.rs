pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMessagesRequest {
    /// A list of file attachments to include with the message, such as images or videos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<CreateMessagesRequestAttachmentsItem>>,
    /// Automatically detect URLs in the message and generate link previews.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_detect_links: Option<bool>,
    /// The unique identifier of the channel or experience to send the message in. For example, 'exp_xxxxx' or 'feed_xxxxx'.
    #[serde(default)]
    pub channel_id: String,
    /// The body of the message in Markdown format. For example, 'Hello **world**'.
    #[serde(default)]
    pub content: String,
    /// A poll to attach to this message, allowing recipients to vote on options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<CreateMessagesRequestPoll>,
    /// The unique identifier of the message this is replying to, creating a threaded reply.
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
            poll: self.poll,
            replying_to_message_id: self.replying_to_message_id,
        })
    }
}
