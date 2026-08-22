pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMembersResponse {
    #[serde(default)]
    pub data: Vec<Member>,
    #[serde(default)]
    pub page_info: ListMembersResponsePageInfo,
}

impl ListMembersResponse {
    pub fn builder() -> ListMembersResponseBuilder {
        <ListMembersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMembersResponseBuilder {
    data: Option<Vec<Member>>,
    page_info: Option<ListMembersResponsePageInfo>,
}

impl ListMembersResponseBuilder {
    pub fn data(mut self, value: Vec<Member>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListMembersResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMembersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListMembersResponseBuilder::data)
    /// - [`page_info`](ListMembersResponseBuilder::page_info)
    pub fn build(self) -> Result<ListMembersResponse, BuildError> {
        Ok(ListMembersResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
