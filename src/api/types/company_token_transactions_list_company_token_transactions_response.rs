pub use crate::prelude::*;

/// The connection type for CompanyTokenTransaction.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListCompanyTokenTransactionsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<CompanyTokenTransactionListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListCompanyTokenTransactionsResponse {
    pub fn builder() -> ListCompanyTokenTransactionsResponseBuilder {
        <ListCompanyTokenTransactionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCompanyTokenTransactionsResponseBuilder {
    data: Option<Vec<CompanyTokenTransactionListItem>>,
    page_info: Option<PageInfo>,
}

impl ListCompanyTokenTransactionsResponseBuilder {
    pub fn data(mut self, value: Vec<CompanyTokenTransactionListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCompanyTokenTransactionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListCompanyTokenTransactionsResponseBuilder::data)
    /// - [`page_info`](ListCompanyTokenTransactionsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListCompanyTokenTransactionsResponse, BuildError> {
        Ok(ListCompanyTokenTransactionsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
