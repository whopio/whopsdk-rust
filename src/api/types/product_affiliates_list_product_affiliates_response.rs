pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListProductAffiliatesResponse {
    #[serde(default)]
    pub data: Vec<ProductAffiliate>,
    #[serde(default)]
    pub page_info: ListProductAffiliatesResponsePageInfo,
    /// Matching product affiliates, or `null` past 10,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

impl ListProductAffiliatesResponse {
    pub fn builder() -> ListProductAffiliatesResponseBuilder {
        <ListProductAffiliatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListProductAffiliatesResponseBuilder {
    data: Option<Vec<ProductAffiliate>>,
    page_info: Option<ListProductAffiliatesResponsePageInfo>,
    total_count: Option<i64>,
}

impl ListProductAffiliatesResponseBuilder {
    pub fn data(mut self, value: Vec<ProductAffiliate>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListProductAffiliatesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListProductAffiliatesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListProductAffiliatesResponseBuilder::data)
    /// - [`page_info`](ListProductAffiliatesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListProductAffiliatesResponse, BuildError> {
        Ok(ListProductAffiliatesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
            total_count: self.total_count,
        })
    }
}
