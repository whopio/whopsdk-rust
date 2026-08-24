pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSocialAccountsResponse {
    #[serde(default)]
    pub data: Vec<SocialAccount>,
    #[serde(default)]
    pub page_info: ListSocialAccountsResponsePageInfo,
}

impl ListSocialAccountsResponse {
    pub fn builder() -> ListSocialAccountsResponseBuilder {
        <ListSocialAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSocialAccountsResponseBuilder {
    data: Option<Vec<SocialAccount>>,
    page_info: Option<ListSocialAccountsResponsePageInfo>,
}

impl ListSocialAccountsResponseBuilder {
    pub fn data(mut self, value: Vec<SocialAccount>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListSocialAccountsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSocialAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListSocialAccountsResponseBuilder::data)
    /// - [`page_info`](ListSocialAccountsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListSocialAccountsResponse, BuildError> {
        Ok(ListSocialAccountsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
