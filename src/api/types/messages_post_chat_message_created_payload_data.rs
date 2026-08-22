pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostChatMessageCreatedPayloadData {
    pub audience: PostChatMessageCreatedPayloadDataAudience,
    pub channel: PostChatMessageCreatedPayloadDataChannel,
    pub message: Message,
    #[serde(default)]
    pub reason: String,
}

impl PostChatMessageCreatedPayloadData {
    pub fn builder() -> PostChatMessageCreatedPayloadDataBuilder {
        <PostChatMessageCreatedPayloadDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostChatMessageCreatedPayloadDataBuilder {
    audience: Option<PostChatMessageCreatedPayloadDataAudience>,
    channel: Option<PostChatMessageCreatedPayloadDataChannel>,
    message: Option<Message>,
    reason: Option<String>,
}

impl PostChatMessageCreatedPayloadDataBuilder {
    pub fn audience(mut self, value: PostChatMessageCreatedPayloadDataAudience) -> Self {
        self.audience = Some(value);
        self
    }

    pub fn channel(mut self, value: PostChatMessageCreatedPayloadDataChannel) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn message(mut self, value: Message) -> Self {
        self.message = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostChatMessageCreatedPayloadData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audience`](PostChatMessageCreatedPayloadDataBuilder::audience)
    /// - [`channel`](PostChatMessageCreatedPayloadDataBuilder::channel)
    /// - [`message`](PostChatMessageCreatedPayloadDataBuilder::message)
    /// - [`reason`](PostChatMessageCreatedPayloadDataBuilder::reason)
    pub fn build(self) -> Result<PostChatMessageCreatedPayloadData, BuildError> {
        Ok(PostChatMessageCreatedPayloadData {
            audience: self
                .audience
                .ok_or_else(|| BuildError::missing_field("audience"))?,
            channel: self
                .channel
                .ok_or_else(|| BuildError::missing_field("channel"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
