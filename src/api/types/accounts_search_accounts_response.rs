pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchAccountsResponse {
    #[serde(default)]
    pub data: Vec<AccountSearchResult>,
    #[serde(default)]
    pub page_info: SearchAccountsResponsePageInfo,
}

impl SearchAccountsResponse {
    pub fn builder() -> SearchAccountsResponseBuilder {
        <SearchAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchAccountsResponseBuilder {
    data: Option<Vec<AccountSearchResult>>,
    page_info: Option<SearchAccountsResponsePageInfo>,
}

impl SearchAccountsResponseBuilder {
    pub fn data(mut self, value: Vec<AccountSearchResult>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: SearchAccountsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](SearchAccountsResponseBuilder::data)
    /// - [`page_info`](SearchAccountsResponseBuilder::page_info)
    pub fn build(self) -> Result<SearchAccountsResponse, BuildError> {
        Ok(SearchAccountsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
