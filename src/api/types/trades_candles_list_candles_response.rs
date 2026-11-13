pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCandlesResponse {
    #[serde(default)]
    pub data: Vec<Candle>,
    #[serde(default)]
    pub page_info: ListCandlesResponsePageInfo,
}

impl ListCandlesResponse {
    pub fn builder() -> ListCandlesResponseBuilder {
        <ListCandlesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCandlesResponseBuilder {
    data: Option<Vec<Candle>>,
    page_info: Option<ListCandlesResponsePageInfo>,
}

impl ListCandlesResponseBuilder {
    pub fn data(mut self, value: Vec<Candle>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListCandlesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCandlesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListCandlesResponseBuilder::data)
    /// - [`page_info`](ListCandlesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListCandlesResponse, BuildError> {
        Ok(ListCandlesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
