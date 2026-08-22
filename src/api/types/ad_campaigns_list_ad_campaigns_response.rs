pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAdCampaignsResponse {
    #[serde(default)]
    pub data: Vec<AdCampaign>,
    #[serde(default)]
    pub page_info: ListAdCampaignsResponsePageInfo,
}

impl ListAdCampaignsResponse {
    pub fn builder() -> ListAdCampaignsResponseBuilder {
        <ListAdCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAdCampaignsResponseBuilder {
    data: Option<Vec<AdCampaign>>,
    page_info: Option<ListAdCampaignsResponsePageInfo>,
}

impl ListAdCampaignsResponseBuilder {
    pub fn data(mut self, value: Vec<AdCampaign>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListAdCampaignsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAdCampaignsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAdCampaignsResponseBuilder::data)
    /// - [`page_info`](ListAdCampaignsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAdCampaignsResponse, BuildError> {
        Ok(ListAdCampaignsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
