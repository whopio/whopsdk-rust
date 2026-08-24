pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMembershipsResponse {
    #[serde(default)]
    pub data: Vec<Membership>,
    #[serde(default)]
    pub page_info: ListMembershipsResponsePageInfo,
}

impl ListMembershipsResponse {
    pub fn builder() -> ListMembershipsResponseBuilder {
        <ListMembershipsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMembershipsResponseBuilder {
    data: Option<Vec<Membership>>,
    page_info: Option<ListMembershipsResponsePageInfo>,
}

impl ListMembershipsResponseBuilder {
    pub fn data(mut self, value: Vec<Membership>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListMembershipsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMembershipsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListMembershipsResponseBuilder::data)
    /// - [`page_info`](ListMembershipsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListMembershipsResponse, BuildError> {
        Ok(ListMembershipsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
