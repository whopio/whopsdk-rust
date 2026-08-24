pub use crate::prelude::*;

/// The connection type for FeeMarkup.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListFeeMarkupsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<FeeMarkupListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListFeeMarkupsResponse {
    pub fn builder() -> ListFeeMarkupsResponseBuilder {
        <ListFeeMarkupsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFeeMarkupsResponseBuilder {
    data: Option<Vec<FeeMarkupListItem>>,
    page_info: Option<PageInfo>,
}

impl ListFeeMarkupsResponseBuilder {
    pub fn data(mut self, value: Vec<FeeMarkupListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFeeMarkupsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListFeeMarkupsResponseBuilder::data)
    /// - [`page_info`](ListFeeMarkupsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListFeeMarkupsResponse, BuildError> {
        Ok(ListFeeMarkupsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
