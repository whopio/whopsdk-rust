pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListProductsResponse {
    #[serde(default)]
    pub data: Vec<ProductListItem>,
    #[serde(default)]
    pub page_info: ListProductsResponsePageInfo,
}

impl ListProductsResponse {
    pub fn builder() -> ListProductsResponseBuilder {
        <ListProductsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListProductsResponseBuilder {
    data: Option<Vec<ProductListItem>>,
    page_info: Option<ListProductsResponsePageInfo>,
}

impl ListProductsResponseBuilder {
    pub fn data(mut self, value: Vec<ProductListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListProductsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListProductsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListProductsResponseBuilder::data)
    /// - [`page_info`](ListProductsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListProductsResponse, BuildError> {
        Ok(ListProductsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
