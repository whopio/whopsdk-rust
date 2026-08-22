pub use crate::prelude::*;

/// The connection type for Fee.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListFeesPaymentsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<ListFeesPaymentsResponseDataItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListFeesPaymentsResponse {
    pub fn builder() -> ListFeesPaymentsResponseBuilder {
        <ListFeesPaymentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFeesPaymentsResponseBuilder {
    data: Option<Vec<ListFeesPaymentsResponseDataItem>>,
    page_info: Option<PageInfo>,
}

impl ListFeesPaymentsResponseBuilder {
    pub fn data(mut self, value: Vec<ListFeesPaymentsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFeesPaymentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListFeesPaymentsResponseBuilder::data)
    /// - [`page_info`](ListFeesPaymentsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListFeesPaymentsResponse, BuildError> {
        Ok(ListFeesPaymentsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
