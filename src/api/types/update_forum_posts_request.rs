pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateForumPostsRequest {
    /// A replacement list of file attachments for this post, such as images or videos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<UpdateForumPostsRequestAttachmentsItem>>,
    /// The updated body of the post in Markdown format. For example, 'Check out this **update**'. Hidden if the post is paywalled and the viewer has not purchased access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Whether this post should be pinned to the top of the forum. Only top-level posts can be pinned, not comments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pinned: Option<bool>,
    /// The updated title of the post, displayed prominently at the top. Required for paywalled posts as it remains visible to non-purchasers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Controls who can see this forum post, such as members only or public.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ForumPostVisibilityTypes>,
}

impl UpdateForumPostsRequest {
    pub fn builder() -> UpdateForumPostsRequestBuilder {
        <UpdateForumPostsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateForumPostsRequestBuilder {
    attachments: Option<Vec<UpdateForumPostsRequestAttachmentsItem>>,
    content: Option<String>,
    is_pinned: Option<bool>,
    title: Option<String>,
    visibility: Option<ForumPostVisibilityTypes>,
}

impl UpdateForumPostsRequestBuilder {
    pub fn attachments(mut self, value: Vec<UpdateForumPostsRequestAttachmentsItem>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn is_pinned(mut self, value: bool) -> Self {
        self.is_pinned = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: ForumPostVisibilityTypes) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateForumPostsRequest`].
    pub fn build(self) -> Result<UpdateForumPostsRequest, BuildError> {
        Ok(UpdateForumPostsRequest {
            attachments: self.attachments,
            content: self.content,
            is_pinned: self.is_pinned,
            title: self.title,
            visibility: self.visibility,
        })
    }
}
