pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostChatMessageCreatedPayloadDataChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_id: Option<String>,
    #[serde(default)]
    pub id: String,
    pub r#type: PostChatMessageCreatedPayloadDataChannelType,
}

impl PostChatMessageCreatedPayloadDataChannel {
    pub fn builder() -> PostChatMessageCreatedPayloadDataChannelBuilder {
        <PostChatMessageCreatedPayloadDataChannelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostChatMessageCreatedPayloadDataChannelBuilder {
    experience_id: Option<String>,
    id: Option<String>,
    r#type: Option<PostChatMessageCreatedPayloadDataChannelType>,
}

impl PostChatMessageCreatedPayloadDataChannelBuilder {
    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostChatMessageCreatedPayloadDataChannelType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostChatMessageCreatedPayloadDataChannel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostChatMessageCreatedPayloadDataChannelBuilder::id)
    /// - [`r#type`](PostChatMessageCreatedPayloadDataChannelBuilder::r#type)
    pub fn build(self) -> Result<PostChatMessageCreatedPayloadDataChannel, BuildError> {
        Ok(PostChatMessageCreatedPayloadDataChannel {
            experience_id: self.experience_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
