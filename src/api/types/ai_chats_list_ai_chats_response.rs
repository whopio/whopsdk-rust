pub use crate::prelude::*;

/// The connection type for AiChat.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAiChatsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<AiChatListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListAiChatsResponse {
    pub fn builder() -> ListAiChatsResponseBuilder {
        <ListAiChatsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAiChatsResponseBuilder {
    data: Option<Vec<AiChatListItem>>,
    page_info: Option<PageInfo>,
}

impl ListAiChatsResponseBuilder {
    pub fn data(mut self, value: Vec<AiChatListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAiChatsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAiChatsResponseBuilder::data)
    /// - [`page_info`](ListAiChatsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAiChatsResponse, BuildError> {
        Ok(ListAiChatsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
