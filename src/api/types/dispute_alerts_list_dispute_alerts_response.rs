pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListDisputeAlertsResponse {
    #[serde(default)]
    pub data: Vec<DisputeAlert>,
    #[serde(default)]
    pub page_info: ListDisputeAlertsResponsePageInfo,
}

impl ListDisputeAlertsResponse {
    pub fn builder() -> ListDisputeAlertsResponseBuilder {
        <ListDisputeAlertsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDisputeAlertsResponseBuilder {
    data: Option<Vec<DisputeAlert>>,
    page_info: Option<ListDisputeAlertsResponsePageInfo>,
}

impl ListDisputeAlertsResponseBuilder {
    pub fn data(mut self, value: Vec<DisputeAlert>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListDisputeAlertsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDisputeAlertsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListDisputeAlertsResponseBuilder::data)
    /// - [`page_info`](ListDisputeAlertsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListDisputeAlertsResponse, BuildError> {
        Ok(ListDisputeAlertsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
