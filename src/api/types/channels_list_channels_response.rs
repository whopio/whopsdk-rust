pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListChannelsResponse {
    #[serde(default)]
    pub data: Vec<ChannelListItem>,
    #[serde(default)]
    pub page_info: ListChannelsResponsePageInfo,
}

impl ListChannelsResponse {
    pub fn builder() -> ListChannelsResponseBuilder {
        <ListChannelsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListChannelsResponseBuilder {
    data: Option<Vec<ChannelListItem>>,
    page_info: Option<ListChannelsResponsePageInfo>,
}

impl ListChannelsResponseBuilder {
    pub fn data(mut self, value: Vec<ChannelListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListChannelsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListChannelsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListChannelsResponseBuilder::data)
    /// - [`page_info`](ListChannelsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListChannelsResponse, BuildError> {
        Ok(ListChannelsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
