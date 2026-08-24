pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForumPostsListQueryRequest {
    /// Returns the elements in the list that come after the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Returns the elements in the list that come before the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Returns the first _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Returns the last _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// The unique identifier of the experience to list forum posts for.
    #[serde(default)]
    pub experience_id: String,
    /// Whether to include top-level bounty discussion anchors as rich forum items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_bounty_anchors: Option<bool>,
    /// The unique identifier of a parent post to list comments for. When set, returns replies to that post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// Whether to filter for only pinned posts. Set to true to return only pinned posts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
}

impl ForumPostsListQueryRequest {
    pub fn builder() -> ForumPostsListQueryRequestBuilder {
        <ForumPostsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForumPostsListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    experience_id: Option<String>,
    include_bounty_anchors: Option<bool>,
    parent_id: Option<String>,
    pinned: Option<bool>,
}

impl ForumPostsListQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    pub fn include_bounty_anchors(mut self, value: bool) -> Self {
        self.include_bounty_anchors = Some(value);
        self
    }

    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    pub fn pinned(mut self, value: bool) -> Self {
        self.pinned = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForumPostsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`experience_id`](ForumPostsListQueryRequestBuilder::experience_id)
    pub fn build(self) -> Result<ForumPostsListQueryRequest, BuildError> {
        Ok(ForumPostsListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            experience_id: self
                .experience_id
                .ok_or_else(|| BuildError::missing_field("experience_id"))?,
            include_bounty_anchors: self.include_bounty_anchors,
            parent_id: self.parent_id,
            pinned: self.pinned,
        })
    }
}
