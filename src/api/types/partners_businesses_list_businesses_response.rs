pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListBusinessesResponse {
    #[serde(default)]
    pub data: Vec<ListBusinessesResponseDataItem>,
    #[serde(default)]
    pub page_info: ListBusinessesResponsePageInfo,
}

impl ListBusinessesResponse {
    pub fn builder() -> ListBusinessesResponseBuilder {
        <ListBusinessesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBusinessesResponseBuilder {
    data: Option<Vec<ListBusinessesResponseDataItem>>,
    page_info: Option<ListBusinessesResponsePageInfo>,
}

impl ListBusinessesResponseBuilder {
    pub fn data(mut self, value: Vec<ListBusinessesResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListBusinessesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBusinessesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListBusinessesResponseBuilder::data)
    /// - [`page_info`](ListBusinessesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListBusinessesResponse, BuildError> {
        Ok(ListBusinessesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
