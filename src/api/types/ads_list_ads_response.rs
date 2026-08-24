pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAdsResponse {
    #[serde(default)]
    pub data: Vec<Ad>,
    #[serde(default)]
    pub page_info: ListAdsResponsePageInfo,
}

impl ListAdsResponse {
    pub fn builder() -> ListAdsResponseBuilder {
        <ListAdsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAdsResponseBuilder {
    data: Option<Vec<Ad>>,
    page_info: Option<ListAdsResponsePageInfo>,
}

impl ListAdsResponseBuilder {
    pub fn data(mut self, value: Vec<Ad>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListAdsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAdsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAdsResponseBuilder::data)
    /// - [`page_info`](ListAdsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAdsResponse, BuildError> {
        Ok(ListAdsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
