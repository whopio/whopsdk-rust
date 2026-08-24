pub use crate::prelude::*;

/// The connection type for Verification.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListVerificationsIdentityProfileResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<ListVerificationsIdentityProfileResponseDataItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListVerificationsIdentityProfileResponse {
    pub fn builder() -> ListVerificationsIdentityProfileResponseBuilder {
        <ListVerificationsIdentityProfileResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListVerificationsIdentityProfileResponseBuilder {
    data: Option<Vec<ListVerificationsIdentityProfileResponseDataItem>>,
    page_info: Option<PageInfo>,
}

impl ListVerificationsIdentityProfileResponseBuilder {
    pub fn data(mut self, value: Vec<ListVerificationsIdentityProfileResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListVerificationsIdentityProfileResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListVerificationsIdentityProfileResponseBuilder::data)
    /// - [`page_info`](ListVerificationsIdentityProfileResponseBuilder::page_info)
    pub fn build(self) -> Result<ListVerificationsIdentityProfileResponse, BuildError> {
        Ok(ListVerificationsIdentityProfileResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
