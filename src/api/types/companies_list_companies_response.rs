pub use crate::prelude::*;

/// The connection type for PublicCompany.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListCompaniesResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<CompanyListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListCompaniesResponse {
    pub fn builder() -> ListCompaniesResponseBuilder {
        <ListCompaniesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCompaniesResponseBuilder {
    data: Option<Vec<CompanyListItem>>,
    page_info: Option<PageInfo>,
}

impl ListCompaniesResponseBuilder {
    pub fn data(mut self, value: Vec<CompanyListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCompaniesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListCompaniesResponseBuilder::data)
    /// - [`page_info`](ListCompaniesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListCompaniesResponse, BuildError> {
        Ok(ListCompaniesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
