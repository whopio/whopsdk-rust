pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostChatReactionCreatedPayloadDataChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_id: Option<String>,
    #[serde(default)]
    pub id: String,
    pub r#type: PostChatReactionCreatedPayloadDataChannelType,
}

impl PostChatReactionCreatedPayloadDataChannel {
    pub fn builder() -> PostChatReactionCreatedPayloadDataChannelBuilder {
        <PostChatReactionCreatedPayloadDataChannelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostChatReactionCreatedPayloadDataChannelBuilder {
    experience_id: Option<String>,
    id: Option<String>,
    r#type: Option<PostChatReactionCreatedPayloadDataChannelType>,
}

impl PostChatReactionCreatedPayloadDataChannelBuilder {
    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostChatReactionCreatedPayloadDataChannelType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostChatReactionCreatedPayloadDataChannel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostChatReactionCreatedPayloadDataChannelBuilder::id)
    /// - [`r#type`](PostChatReactionCreatedPayloadDataChannelBuilder::r#type)
    pub fn build(self) -> Result<PostChatReactionCreatedPayloadDataChannel, BuildError> {
        Ok(PostChatReactionCreatedPayloadDataChannel {
            experience_id: self.experience_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
