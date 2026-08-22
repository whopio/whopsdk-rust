pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListFinancialActivityResponse {
    #[serde(default)]
    pub data: Vec<LedgerActivity>,
    #[serde(default)]
    pub page_info: ListFinancialActivityResponsePageInfo,
}

impl ListFinancialActivityResponse {
    pub fn builder() -> ListFinancialActivityResponseBuilder {
        <ListFinancialActivityResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFinancialActivityResponseBuilder {
    data: Option<Vec<LedgerActivity>>,
    page_info: Option<ListFinancialActivityResponsePageInfo>,
}

impl ListFinancialActivityResponseBuilder {
    pub fn data(mut self, value: Vec<LedgerActivity>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListFinancialActivityResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFinancialActivityResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListFinancialActivityResponseBuilder::data)
    /// - [`page_info`](ListFinancialActivityResponseBuilder::page_info)
    pub fn build(self) -> Result<ListFinancialActivityResponse, BuildError> {
        Ok(ListFinancialActivityResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
