pub use crate::prelude::*;

/// The connection type for IdentityProfile.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListIdentityProfileResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<IdentityProfileListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListIdentityProfileResponse {
    pub fn builder() -> ListIdentityProfileResponseBuilder {
        <ListIdentityProfileResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListIdentityProfileResponseBuilder {
    data: Option<Vec<IdentityProfileListItem>>,
    page_info: Option<PageInfo>,
}

impl ListIdentityProfileResponseBuilder {
    pub fn data(mut self, value: Vec<IdentityProfileListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListIdentityProfileResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListIdentityProfileResponseBuilder::data)
    /// - [`page_info`](ListIdentityProfileResponseBuilder::page_info)
    pub fn build(self) -> Result<ListIdentityProfileResponse, BuildError> {
        Ok(ListIdentityProfileResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
