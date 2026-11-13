pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListTradesResponse {
    #[serde(default)]
    pub data: Vec<Trade>,
    #[serde(default)]
    pub page_info: ListTradesResponsePageInfo,
}

impl ListTradesResponse {
    pub fn builder() -> ListTradesResponseBuilder {
        <ListTradesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTradesResponseBuilder {
    data: Option<Vec<Trade>>,
    page_info: Option<ListTradesResponsePageInfo>,
}

impl ListTradesResponseBuilder {
    pub fn data(mut self, value: Vec<Trade>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListTradesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTradesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListTradesResponseBuilder::data)
    /// - [`page_info`](ListTradesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListTradesResponse, BuildError> {
        Ok(ListTradesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
