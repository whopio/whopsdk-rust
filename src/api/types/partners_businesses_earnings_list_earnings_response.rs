pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEarningsResponse {
    #[serde(default)]
    pub data: Vec<ListEarningsResponseDataItem>,
    #[serde(default)]
    pub page_info: ListEarningsResponsePageInfo,
}

impl ListEarningsResponse {
    pub fn builder() -> ListEarningsResponseBuilder {
        <ListEarningsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEarningsResponseBuilder {
    data: Option<Vec<ListEarningsResponseDataItem>>,
    page_info: Option<ListEarningsResponsePageInfo>,
}

impl ListEarningsResponseBuilder {
    pub fn data(mut self, value: Vec<ListEarningsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListEarningsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEarningsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListEarningsResponseBuilder::data)
    /// - [`page_info`](ListEarningsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListEarningsResponse, BuildError> {
        Ok(ListEarningsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
