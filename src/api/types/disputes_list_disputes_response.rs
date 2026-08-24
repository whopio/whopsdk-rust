pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListDisputesResponse {
    #[serde(default)]
    pub data: Vec<Dispute>,
    #[serde(default)]
    pub page_info: ListDisputesResponsePageInfo,
}

impl ListDisputesResponse {
    pub fn builder() -> ListDisputesResponseBuilder {
        <ListDisputesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDisputesResponseBuilder {
    data: Option<Vec<Dispute>>,
    page_info: Option<ListDisputesResponsePageInfo>,
}

impl ListDisputesResponseBuilder {
    pub fn data(mut self, value: Vec<Dispute>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListDisputesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDisputesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListDisputesResponseBuilder::data)
    /// - [`page_info`](ListDisputesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListDisputesResponse, BuildError> {
        Ok(ListDisputesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
