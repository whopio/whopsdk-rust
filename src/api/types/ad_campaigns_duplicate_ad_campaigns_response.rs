pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DuplicateAdCampaignsResponse {
    #[serde(default)]
    pub data: Vec<AdCampaign>,
}

impl DuplicateAdCampaignsResponse {
    pub fn builder() -> DuplicateAdCampaignsResponseBuilder {
        <DuplicateAdCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DuplicateAdCampaignsResponseBuilder {
    data: Option<Vec<AdCampaign>>,
}

impl DuplicateAdCampaignsResponseBuilder {
    pub fn data(mut self, value: Vec<AdCampaign>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DuplicateAdCampaignsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](DuplicateAdCampaignsResponseBuilder::data)
    pub fn build(self) -> Result<DuplicateAdCampaignsResponse, BuildError> {
        Ok(DuplicateAdCampaignsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
