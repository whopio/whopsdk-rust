pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListExternalAccountsResponse {
    #[serde(default)]
    pub data: Vec<ExternalAccount>,
    #[serde(default)]
    pub page_info: ListExternalAccountsResponsePageInfo,
}

impl ListExternalAccountsResponse {
    pub fn builder() -> ListExternalAccountsResponseBuilder {
        <ListExternalAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListExternalAccountsResponseBuilder {
    data: Option<Vec<ExternalAccount>>,
    page_info: Option<ListExternalAccountsResponsePageInfo>,
}

impl ListExternalAccountsResponseBuilder {
    pub fn data(mut self, value: Vec<ExternalAccount>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListExternalAccountsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListExternalAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListExternalAccountsResponseBuilder::data)
    /// - [`page_info`](ListExternalAccountsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListExternalAccountsResponse, BuildError> {
        Ok(ListExternalAccountsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
