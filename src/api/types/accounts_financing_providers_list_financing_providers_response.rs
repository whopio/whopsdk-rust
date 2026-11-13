pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListFinancingProvidersResponse {
    #[serde(default)]
    pub data: Vec<FinancingProvider>,
    #[serde(default)]
    pub page_info: ListFinancingProvidersResponsePageInfo,
}

impl ListFinancingProvidersResponse {
    pub fn builder() -> ListFinancingProvidersResponseBuilder {
        <ListFinancingProvidersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFinancingProvidersResponseBuilder {
    data: Option<Vec<FinancingProvider>>,
    page_info: Option<ListFinancingProvidersResponsePageInfo>,
}

impl ListFinancingProvidersResponseBuilder {
    pub fn data(mut self, value: Vec<FinancingProvider>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListFinancingProvidersResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFinancingProvidersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListFinancingProvidersResponseBuilder::data)
    /// - [`page_info`](ListFinancingProvidersResponseBuilder::page_info)
    pub fn build(self) -> Result<ListFinancingProvidersResponse, BuildError> {
        Ok(ListFinancingProvidersResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
