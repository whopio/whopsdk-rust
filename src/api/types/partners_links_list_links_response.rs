pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListLinksResponse {
    /// The partner's standard shareable referral URL.
    #[serde(default)]
    pub base_referral_url: String,
    #[serde(default)]
    pub data: Vec<PartnerRewardLink>,
    #[serde(default)]
    pub page_info: ListLinksResponsePageInfo,
}

impl ListLinksResponse {
    pub fn builder() -> ListLinksResponseBuilder {
        <ListLinksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLinksResponseBuilder {
    base_referral_url: Option<String>,
    data: Option<Vec<PartnerRewardLink>>,
    page_info: Option<ListLinksResponsePageInfo>,
}

impl ListLinksResponseBuilder {
    pub fn base_referral_url(mut self, value: impl Into<String>) -> Self {
        self.base_referral_url = Some(value.into());
        self
    }

    pub fn data(mut self, value: Vec<PartnerRewardLink>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListLinksResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListLinksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`base_referral_url`](ListLinksResponseBuilder::base_referral_url)
    /// - [`data`](ListLinksResponseBuilder::data)
    /// - [`page_info`](ListLinksResponseBuilder::page_info)
    pub fn build(self) -> Result<ListLinksResponse, BuildError> {
        Ok(ListLinksResponse {
            base_referral_url: self
                .base_referral_url
                .ok_or_else(|| BuildError::missing_field("base_referral_url"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
