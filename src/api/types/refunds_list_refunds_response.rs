pub use crate::prelude::*;

/// The connection type for Refund.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListRefundsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<RefundListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListRefundsResponse {
    pub fn builder() -> ListRefundsResponseBuilder {
        <ListRefundsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRefundsResponseBuilder {
    data: Option<Vec<RefundListItem>>,
    page_info: Option<PageInfo>,
}

impl ListRefundsResponseBuilder {
    pub fn data(mut self, value: Vec<RefundListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListRefundsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListRefundsResponseBuilder::data)
    /// - [`page_info`](ListRefundsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListRefundsResponse, BuildError> {
        Ok(ListRefundsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
