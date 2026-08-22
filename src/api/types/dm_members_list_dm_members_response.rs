pub use crate::prelude::*;

/// The connection type for DmsFeedMember.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDmMembersResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<DmMemberListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListDmMembersResponse {
    pub fn builder() -> ListDmMembersResponseBuilder {
        <ListDmMembersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDmMembersResponseBuilder {
    data: Option<Vec<DmMemberListItem>>,
    page_info: Option<PageInfo>,
}

impl ListDmMembersResponseBuilder {
    pub fn data(mut self, value: Vec<DmMemberListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDmMembersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListDmMembersResponseBuilder::data)
    /// - [`page_info`](ListDmMembersResponseBuilder::page_info)
    pub fn build(self) -> Result<ListDmMembersResponse, BuildError> {
        Ok(ListDmMembersResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
