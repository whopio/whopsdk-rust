pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSendsResponse {
    #[serde(default)]
    pub data: Vec<InvoiceSend>,
    #[serde(default)]
    pub page_info: ListSendsResponsePageInfo,
}

impl ListSendsResponse {
    pub fn builder() -> ListSendsResponseBuilder {
        <ListSendsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSendsResponseBuilder {
    data: Option<Vec<InvoiceSend>>,
    page_info: Option<ListSendsResponsePageInfo>,
}

impl ListSendsResponseBuilder {
    pub fn data(mut self, value: Vec<InvoiceSend>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListSendsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSendsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListSendsResponseBuilder::data)
    /// - [`page_info`](ListSendsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListSendsResponse, BuildError> {
        Ok(ListSendsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
