pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListTransfersResponse {
    #[serde(default)]
    pub data: Vec<ListTransfersResponseDataItem>,
    #[serde(default)]
    pub page_info: ListTransfersResponsePageInfo,
}

impl ListTransfersResponse {
    pub fn builder() -> ListTransfersResponseBuilder {
        <ListTransfersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTransfersResponseBuilder {
    data: Option<Vec<ListTransfersResponseDataItem>>,
    page_info: Option<ListTransfersResponsePageInfo>,
}

impl ListTransfersResponseBuilder {
    pub fn data(mut self, value: Vec<ListTransfersResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListTransfersResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTransfersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListTransfersResponseBuilder::data)
    /// - [`page_info`](ListTransfersResponseBuilder::page_info)
    pub fn build(self) -> Result<ListTransfersResponse, BuildError> {
        Ok(ListTransfersResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
