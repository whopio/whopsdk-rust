pub use crate::prelude::*;

/// The connection type for ForumFeed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListForumsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<ForumListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListForumsResponse {
    pub fn builder() -> ListForumsResponseBuilder {
        <ListForumsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListForumsResponseBuilder {
    data: Option<Vec<ForumListItem>>,
    page_info: Option<PageInfo>,
}

impl ListForumsResponseBuilder {
    pub fn data(mut self, value: Vec<ForumListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListForumsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListForumsResponseBuilder::data)
    /// - [`page_info`](ListForumsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListForumsResponse, BuildError> {
        Ok(ListForumsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
