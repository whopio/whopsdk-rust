pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListFinancingApplicationsResponse {
    #[serde(default)]
    pub data: Vec<FinancingApplication>,
    #[serde(default)]
    pub page_info: ListFinancingApplicationsResponsePageInfo,
}

impl ListFinancingApplicationsResponse {
    pub fn builder() -> ListFinancingApplicationsResponseBuilder {
        <ListFinancingApplicationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFinancingApplicationsResponseBuilder {
    data: Option<Vec<FinancingApplication>>,
    page_info: Option<ListFinancingApplicationsResponsePageInfo>,
}

impl ListFinancingApplicationsResponseBuilder {
    pub fn data(mut self, value: Vec<FinancingApplication>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListFinancingApplicationsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFinancingApplicationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListFinancingApplicationsResponseBuilder::data)
    /// - [`page_info`](ListFinancingApplicationsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListFinancingApplicationsResponse, BuildError> {
        Ok(ListFinancingApplicationsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
