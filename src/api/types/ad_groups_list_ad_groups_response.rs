pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAdGroupsResponse {
    #[serde(default)]
    pub data: Vec<AdGroup>,
    #[serde(default)]
    pub page_info: ListAdGroupsResponsePageInfo,
}

impl ListAdGroupsResponse {
    pub fn builder() -> ListAdGroupsResponseBuilder {
        <ListAdGroupsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAdGroupsResponseBuilder {
    data: Option<Vec<AdGroup>>,
    page_info: Option<ListAdGroupsResponsePageInfo>,
}

impl ListAdGroupsResponseBuilder {
    pub fn data(mut self, value: Vec<AdGroup>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListAdGroupsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAdGroupsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAdGroupsResponseBuilder::data)
    /// - [`page_info`](ListAdGroupsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAdGroupsResponse, BuildError> {
        Ok(ListAdGroupsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
