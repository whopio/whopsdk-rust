pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListCardTransactionsResponse {
    #[serde(default)]
    pub data: Vec<CardTransaction>,
    #[serde(default)]
    pub page_info: ListCardTransactionsResponsePageInfo,
}

impl ListCardTransactionsResponse {
    pub fn builder() -> ListCardTransactionsResponseBuilder {
        <ListCardTransactionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCardTransactionsResponseBuilder {
    data: Option<Vec<CardTransaction>>,
    page_info: Option<ListCardTransactionsResponsePageInfo>,
}

impl ListCardTransactionsResponseBuilder {
    pub fn data(mut self, value: Vec<CardTransaction>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListCardTransactionsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCardTransactionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListCardTransactionsResponseBuilder::data)
    /// - [`page_info`](ListCardTransactionsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListCardTransactionsResponse, BuildError> {
        Ok(ListCardTransactionsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
