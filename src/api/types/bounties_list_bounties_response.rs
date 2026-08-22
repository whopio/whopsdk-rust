pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListBountiesResponse {
    #[serde(default)]
    pub data: Vec<BountyListItem>,
    #[serde(default)]
    pub page_info: ListBountiesResponsePageInfo,
}

impl ListBountiesResponse {
    pub fn builder() -> ListBountiesResponseBuilder {
        <ListBountiesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBountiesResponseBuilder {
    data: Option<Vec<BountyListItem>>,
    page_info: Option<ListBountiesResponsePageInfo>,
}

impl ListBountiesResponseBuilder {
    pub fn data(mut self, value: Vec<BountyListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListBountiesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBountiesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListBountiesResponseBuilder::data)
    /// - [`page_info`](ListBountiesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListBountiesResponse, BuildError> {
        Ok(ListBountiesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
