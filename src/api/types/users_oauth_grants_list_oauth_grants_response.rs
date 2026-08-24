pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOauthGrantsResponse {
    #[serde(default)]
    pub data: Vec<OauthGrant>,
    #[serde(default)]
    pub page_info: ListOauthGrantsResponsePageInfo,
}

impl ListOauthGrantsResponse {
    pub fn builder() -> ListOauthGrantsResponseBuilder {
        <ListOauthGrantsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOauthGrantsResponseBuilder {
    data: Option<Vec<OauthGrant>>,
    page_info: Option<ListOauthGrantsResponsePageInfo>,
}

impl ListOauthGrantsResponseBuilder {
    pub fn data(mut self, value: Vec<OauthGrant>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListOauthGrantsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOauthGrantsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListOauthGrantsResponseBuilder::data)
    /// - [`page_info`](ListOauthGrantsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListOauthGrantsResponse, BuildError> {
        Ok(ListOauthGrantsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
