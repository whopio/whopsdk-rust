pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAccountsResponse {
    #[serde(default)]
    pub data: Vec<Account>,
    #[serde(default)]
    pub page_info: ListAccountsResponsePageInfo,
}

impl ListAccountsResponse {
    pub fn builder() -> ListAccountsResponseBuilder {
        <ListAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAccountsResponseBuilder {
    data: Option<Vec<Account>>,
    page_info: Option<ListAccountsResponsePageInfo>,
}

impl ListAccountsResponseBuilder {
    pub fn data(mut self, value: Vec<Account>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListAccountsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAccountsResponseBuilder::data)
    /// - [`page_info`](ListAccountsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAccountsResponse, BuildError> {
        Ok(ListAccountsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
