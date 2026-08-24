pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSupportedMethodsResponse {
    #[serde(default)]
    pub data: Vec<ListSupportedMethodsResponseDataItem>,
    #[serde(default)]
    pub page_info: ListSupportedMethodsResponsePageInfo,
}

impl ListSupportedMethodsResponse {
    pub fn builder() -> ListSupportedMethodsResponseBuilder {
        <ListSupportedMethodsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSupportedMethodsResponseBuilder {
    data: Option<Vec<ListSupportedMethodsResponseDataItem>>,
    page_info: Option<ListSupportedMethodsResponsePageInfo>,
}

impl ListSupportedMethodsResponseBuilder {
    pub fn data(mut self, value: Vec<ListSupportedMethodsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListSupportedMethodsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSupportedMethodsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListSupportedMethodsResponseBuilder::data)
    /// - [`page_info`](ListSupportedMethodsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListSupportedMethodsResponse, BuildError> {
        Ok(ListSupportedMethodsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
