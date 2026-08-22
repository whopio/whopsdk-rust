pub use crate::prelude::*;

/// The connection type for ChatFeed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListChatChannelsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<ChatChannelListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListChatChannelsResponse {
    pub fn builder() -> ListChatChannelsResponseBuilder {
        <ListChatChannelsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListChatChannelsResponseBuilder {
    data: Option<Vec<ChatChannelListItem>>,
    page_info: Option<PageInfo>,
}

impl ListChatChannelsResponseBuilder {
    pub fn data(mut self, value: Vec<ChatChannelListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListChatChannelsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListChatChannelsResponseBuilder::data)
    /// - [`page_info`](ListChatChannelsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListChatChannelsResponse, BuildError> {
        Ok(ListChatChannelsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
