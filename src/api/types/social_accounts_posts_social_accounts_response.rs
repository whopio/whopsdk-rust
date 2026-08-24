pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostsSocialAccountsResponse {
    #[serde(default)]
    pub data: Vec<SocialAccountPost>,
    #[serde(default)]
    pub page_info: PostsSocialAccountsResponsePageInfo,
}

impl PostsSocialAccountsResponse {
    pub fn builder() -> PostsSocialAccountsResponseBuilder {
        <PostsSocialAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostsSocialAccountsResponseBuilder {
    data: Option<Vec<SocialAccountPost>>,
    page_info: Option<PostsSocialAccountsResponsePageInfo>,
}

impl PostsSocialAccountsResponseBuilder {
    pub fn data(mut self, value: Vec<SocialAccountPost>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PostsSocialAccountsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostsSocialAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](PostsSocialAccountsResponseBuilder::data)
    /// - [`page_info`](PostsSocialAccountsResponseBuilder::page_info)
    pub fn build(self) -> Result<PostsSocialAccountsResponse, BuildError> {
        Ok(PostsSocialAccountsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
