pub use crate::prelude::*;

/// The connection type for Lead.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListLeadsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<LeadListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListLeadsResponse {
    pub fn builder() -> ListLeadsResponseBuilder {
        <ListLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLeadsResponseBuilder {
    data: Option<Vec<LeadListItem>>,
    page_info: Option<PageInfo>,
}

impl ListLeadsResponseBuilder {
    pub fn data(mut self, value: Vec<LeadListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListLeadsResponseBuilder::data)
    /// - [`page_info`](ListLeadsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListLeadsResponse, BuildError> {
        Ok(ListLeadsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
