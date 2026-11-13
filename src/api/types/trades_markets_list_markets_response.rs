pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMarketsResponse {
    #[serde(default)]
    pub data: Vec<Market>,
    #[serde(default)]
    pub page_info: ListMarketsResponsePageInfo,
}

impl ListMarketsResponse {
    pub fn builder() -> ListMarketsResponseBuilder {
        <ListMarketsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMarketsResponseBuilder {
    data: Option<Vec<Market>>,
    page_info: Option<ListMarketsResponsePageInfo>,
}

impl ListMarketsResponseBuilder {
    pub fn data(mut self, value: Vec<Market>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListMarketsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMarketsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListMarketsResponseBuilder::data)
    /// - [`page_info`](ListMarketsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListMarketsResponse, BuildError> {
        Ok(ListMarketsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
