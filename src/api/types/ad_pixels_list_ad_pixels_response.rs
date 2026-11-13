pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAdPixelsResponse {
    #[serde(default)]
    pub data: Vec<AdPixel>,
    #[serde(default)]
    pub page_info: ListAdPixelsResponsePageInfo,
}

impl ListAdPixelsResponse {
    pub fn builder() -> ListAdPixelsResponseBuilder {
        <ListAdPixelsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAdPixelsResponseBuilder {
    data: Option<Vec<AdPixel>>,
    page_info: Option<ListAdPixelsResponsePageInfo>,
}

impl ListAdPixelsResponseBuilder {
    pub fn data(mut self, value: Vec<AdPixel>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListAdPixelsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAdPixelsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAdPixelsResponseBuilder::data)
    /// - [`page_info`](ListAdPixelsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAdPixelsResponse, BuildError> {
        Ok(ListAdPixelsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
