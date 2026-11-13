pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListChannelMembersResponse {
    #[serde(default)]
    pub data: Vec<ChannelMember>,
    #[serde(default)]
    pub page_info: ListChannelMembersResponsePageInfo,
}

impl ListChannelMembersResponse {
    pub fn builder() -> ListChannelMembersResponseBuilder {
        <ListChannelMembersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListChannelMembersResponseBuilder {
    data: Option<Vec<ChannelMember>>,
    page_info: Option<ListChannelMembersResponsePageInfo>,
}

impl ListChannelMembersResponseBuilder {
    pub fn data(mut self, value: Vec<ChannelMember>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListChannelMembersResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListChannelMembersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListChannelMembersResponseBuilder::data)
    /// - [`page_info`](ListChannelMembersResponseBuilder::page_info)
    pub fn build(self) -> Result<ListChannelMembersResponse, BuildError> {
        Ok(ListChannelMembersResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
