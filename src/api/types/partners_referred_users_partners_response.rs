pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReferredUsersPartnersResponse {
    #[serde(default)]
    pub data: Vec<ReferredUsersPartnersResponseDataItem>,
    #[serde(default)]
    pub page_info: ReferredUsersPartnersResponsePageInfo,
}

impl ReferredUsersPartnersResponse {
    pub fn builder() -> ReferredUsersPartnersResponseBuilder {
        <ReferredUsersPartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferredUsersPartnersResponseBuilder {
    data: Option<Vec<ReferredUsersPartnersResponseDataItem>>,
    page_info: Option<ReferredUsersPartnersResponsePageInfo>,
}

impl ReferredUsersPartnersResponseBuilder {
    pub fn data(mut self, value: Vec<ReferredUsersPartnersResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ReferredUsersPartnersResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReferredUsersPartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ReferredUsersPartnersResponseBuilder::data)
    /// - [`page_info`](ReferredUsersPartnersResponseBuilder::page_info)
    pub fn build(self) -> Result<ReferredUsersPartnersResponse, BuildError> {
        Ok(ReferredUsersPartnersResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
