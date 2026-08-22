pub use crate::prelude::*;

/// The connection type for ForumPost.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListForumPostsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<ForumPostListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListForumPostsResponse {
    pub fn builder() -> ListForumPostsResponseBuilder {
        <ListForumPostsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListForumPostsResponseBuilder {
    data: Option<Vec<ForumPostListItem>>,
    page_info: Option<PageInfo>,
}

impl ListForumPostsResponseBuilder {
    pub fn data(mut self, value: Vec<ForumPostListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListForumPostsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListForumPostsResponseBuilder::data)
    /// - [`page_info`](ListForumPostsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListForumPostsResponse, BuildError> {
        Ok(ListForumPostsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
