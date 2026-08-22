pub use crate::prelude::*;

/// The connection type for DmsFeed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSupportChannelsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<SupportChannelListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListSupportChannelsResponse {
    pub fn builder() -> ListSupportChannelsResponseBuilder {
        <ListSupportChannelsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSupportChannelsResponseBuilder {
    data: Option<Vec<SupportChannelListItem>>,
    page_info: Option<PageInfo>,
}

impl ListSupportChannelsResponseBuilder {
    pub fn data(mut self, value: Vec<SupportChannelListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSupportChannelsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListSupportChannelsResponseBuilder::data)
    /// - [`page_info`](ListSupportChannelsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListSupportChannelsResponse, BuildError> {
        Ok(ListSupportChannelsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
