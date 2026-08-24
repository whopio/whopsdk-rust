pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostChatMessageCreatedPayloadDataAudience {
    pub r#type: PostChatMessageCreatedPayloadDataAudienceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

impl PostChatMessageCreatedPayloadDataAudience {
    pub fn builder() -> PostChatMessageCreatedPayloadDataAudienceBuilder {
        <PostChatMessageCreatedPayloadDataAudienceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostChatMessageCreatedPayloadDataAudienceBuilder {
    r#type: Option<PostChatMessageCreatedPayloadDataAudienceType>,
    user_ids: Option<Vec<String>>,
}

impl PostChatMessageCreatedPayloadDataAudienceBuilder {
    pub fn r#type(mut self, value: PostChatMessageCreatedPayloadDataAudienceType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn user_ids(mut self, value: Vec<String>) -> Self {
        self.user_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostChatMessageCreatedPayloadDataAudience`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PostChatMessageCreatedPayloadDataAudienceBuilder::r#type)
    pub fn build(self) -> Result<PostChatMessageCreatedPayloadDataAudience, BuildError> {
        Ok(PostChatMessageCreatedPayloadDataAudience {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            user_ids: self.user_ids,
        })
    }
}
