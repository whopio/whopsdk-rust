pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostChatReactionCreatedPayloadDataAudience {
    pub r#type: PostChatReactionCreatedPayloadDataAudienceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

impl PostChatReactionCreatedPayloadDataAudience {
    pub fn builder() -> PostChatReactionCreatedPayloadDataAudienceBuilder {
        <PostChatReactionCreatedPayloadDataAudienceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostChatReactionCreatedPayloadDataAudienceBuilder {
    r#type: Option<PostChatReactionCreatedPayloadDataAudienceType>,
    user_ids: Option<Vec<String>>,
}

impl PostChatReactionCreatedPayloadDataAudienceBuilder {
    pub fn r#type(mut self, value: PostChatReactionCreatedPayloadDataAudienceType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn user_ids(mut self, value: Vec<String>) -> Self {
        self.user_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostChatReactionCreatedPayloadDataAudience`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PostChatReactionCreatedPayloadDataAudienceBuilder::r#type)
    pub fn build(self) -> Result<PostChatReactionCreatedPayloadDataAudience, BuildError> {
        Ok(PostChatReactionCreatedPayloadDataAudience {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            user_ids: self.user_ids,
        })
    }
}
