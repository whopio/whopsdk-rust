pub use crate::prelude::*;

/// The connection type for Affiliate.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAffiliatesResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<AffiliateListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListAffiliatesResponse {
    pub fn builder() -> ListAffiliatesResponseBuilder {
        <ListAffiliatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAffiliatesResponseBuilder {
    data: Option<Vec<AffiliateListItem>>,
    page_info: Option<PageInfo>,
}

impl ListAffiliatesResponseBuilder {
    pub fn data(mut self, value: Vec<AffiliateListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAffiliatesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAffiliatesResponseBuilder::data)
    /// - [`page_info`](ListAffiliatesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAffiliatesResponse, BuildError> {
        Ok(ListAffiliatesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
