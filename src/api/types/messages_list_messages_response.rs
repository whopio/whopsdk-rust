pub use crate::prelude::*;

/// The connection type for DmsPost.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMessagesResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<MessageListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListMessagesResponse {
    pub fn builder() -> ListMessagesResponseBuilder {
        <ListMessagesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMessagesResponseBuilder {
    data: Option<Vec<MessageListItem>>,
    page_info: Option<PageInfo>,
}

impl ListMessagesResponseBuilder {
    pub fn data(mut self, value: Vec<MessageListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMessagesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListMessagesResponseBuilder::data)
    /// - [`page_info`](ListMessagesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListMessagesResponse, BuildError> {
        Ok(ListMessagesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
