pub use crate::prelude::*;

/// The connection type for DmsFeed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDmChannelsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<DmChannelListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListDmChannelsResponse {
    pub fn builder() -> ListDmChannelsResponseBuilder {
        <ListDmChannelsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDmChannelsResponseBuilder {
    data: Option<Vec<DmChannelListItem>>,
    page_info: Option<PageInfo>,
}

impl ListDmChannelsResponseBuilder {
    pub fn data(mut self, value: Vec<DmChannelListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDmChannelsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListDmChannelsResponseBuilder::data)
    /// - [`page_info`](ListDmChannelsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListDmChannelsResponse, BuildError> {
        Ok(ListDmChannelsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
