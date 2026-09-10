pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListDomainsResponse {
    #[serde(default)]
    pub data: Vec<Domain>,
    #[serde(default)]
    pub page_info: ListDomainsResponsePageInfo,
}

impl ListDomainsResponse {
    pub fn builder() -> ListDomainsResponseBuilder {
        <ListDomainsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDomainsResponseBuilder {
    data: Option<Vec<Domain>>,
    page_info: Option<ListDomainsResponsePageInfo>,
}

impl ListDomainsResponseBuilder {
    pub fn data(mut self, value: Vec<Domain>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListDomainsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDomainsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListDomainsResponseBuilder::data)
    /// - [`page_info`](ListDomainsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListDomainsResponse, BuildError> {
        Ok(ListDomainsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
