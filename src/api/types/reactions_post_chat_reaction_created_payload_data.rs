pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostChatReactionCreatedPayloadData {
    pub audience: PostChatReactionCreatedPayloadDataAudience,
    pub channel: PostChatReactionCreatedPayloadDataChannel,
    pub message: Message,
    #[serde(default)]
    pub reaction: Reaction,
    #[serde(default)]
    pub reason: String,
}

impl PostChatReactionCreatedPayloadData {
    pub fn builder() -> PostChatReactionCreatedPayloadDataBuilder {
        <PostChatReactionCreatedPayloadDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostChatReactionCreatedPayloadDataBuilder {
    audience: Option<PostChatReactionCreatedPayloadDataAudience>,
    channel: Option<PostChatReactionCreatedPayloadDataChannel>,
    message: Option<Message>,
    reaction: Option<Reaction>,
    reason: Option<String>,
}

impl PostChatReactionCreatedPayloadDataBuilder {
    pub fn audience(mut self, value: PostChatReactionCreatedPayloadDataAudience) -> Self {
        self.audience = Some(value);
        self
    }

    pub fn channel(mut self, value: PostChatReactionCreatedPayloadDataChannel) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn message(mut self, value: Message) -> Self {
        self.message = Some(value);
        self
    }

    pub fn reaction(mut self, value: Reaction) -> Self {
        self.reaction = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostChatReactionCreatedPayloadData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audience`](PostChatReactionCreatedPayloadDataBuilder::audience)
    /// - [`channel`](PostChatReactionCreatedPayloadDataBuilder::channel)
    /// - [`message`](PostChatReactionCreatedPayloadDataBuilder::message)
    /// - [`reaction`](PostChatReactionCreatedPayloadDataBuilder::reaction)
    /// - [`reason`](PostChatReactionCreatedPayloadDataBuilder::reason)
    pub fn build(self) -> Result<PostChatReactionCreatedPayloadData, BuildError> {
        Ok(PostChatReactionCreatedPayloadData {
            audience: self
                .audience
                .ok_or_else(|| BuildError::missing_field("audience"))?,
            channel: self
                .channel
                .ok_or_else(|| BuildError::missing_field("channel"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            reaction: self
                .reaction
                .ok_or_else(|| BuildError::missing_field("reaction"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
