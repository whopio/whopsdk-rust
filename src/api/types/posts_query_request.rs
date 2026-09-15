pub use crate::prelude::*;

/// Query parameters for posts
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostsQueryRequest {
    /// The Account (a biz_ identifier) the social account is connected to.
    #[serde(default)]
    pub account_id: String,
    /// Return only the single post with this platform id, instead of the full list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_id: Option<String>,
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl PostsQueryRequest {
    pub fn builder() -> PostsQueryRequestBuilder {
        <PostsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostsQueryRequestBuilder {
    account_id: Option<String>,
    post_id: Option<String>,
    first: Option<i64>,
    after: Option<String>,
}

impl PostsQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn post_id(mut self, value: impl Into<String>) -> Self {
        self.post_id = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](PostsQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<PostsQueryRequest, BuildError> {
        Ok(PostsQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            post_id: self.post_id,
            first: self.first,
            after: self.after,
        })
    }
}
