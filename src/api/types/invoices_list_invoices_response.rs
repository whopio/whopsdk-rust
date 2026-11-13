pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListInvoicesResponse {
    #[serde(default)]
    pub data: Vec<Invoice>,
    #[serde(default)]
    pub page_info: ListInvoicesResponsePageInfo,
}

impl ListInvoicesResponse {
    pub fn builder() -> ListInvoicesResponseBuilder {
        <ListInvoicesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListInvoicesResponseBuilder {
    data: Option<Vec<Invoice>>,
    page_info: Option<ListInvoicesResponsePageInfo>,
}

impl ListInvoicesResponseBuilder {
    pub fn data(mut self, value: Vec<Invoice>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListInvoicesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListInvoicesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListInvoicesResponseBuilder::data)
    /// - [`page_info`](ListInvoicesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListInvoicesResponse, BuildError> {
        Ok(ListInvoicesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
