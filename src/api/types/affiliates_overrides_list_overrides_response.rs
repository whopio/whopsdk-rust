pub use crate::prelude::*;

/// The connection type for AffiliateOverride.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListOverridesResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<ListOverridesResponseDataItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListOverridesResponse {
    pub fn builder() -> ListOverridesResponseBuilder {
        <ListOverridesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOverridesResponseBuilder {
    data: Option<Vec<ListOverridesResponseDataItem>>,
    page_info: Option<PageInfo>,
}

impl ListOverridesResponseBuilder {
    pub fn data(mut self, value: Vec<ListOverridesResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOverridesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListOverridesResponseBuilder::data)
    /// - [`page_info`](ListOverridesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListOverridesResponse, BuildError> {
        Ok(ListOverridesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
