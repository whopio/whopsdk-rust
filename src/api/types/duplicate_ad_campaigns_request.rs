pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DuplicateAdCampaignsRequest {
    /// Number of copies to create (1-10). Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl DuplicateAdCampaignsRequest {
    pub fn builder() -> DuplicateAdCampaignsRequestBuilder {
        <DuplicateAdCampaignsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DuplicateAdCampaignsRequestBuilder {
    count: Option<i64>,
}

impl DuplicateAdCampaignsRequestBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DuplicateAdCampaignsRequest`].
    pub fn build(self) -> Result<DuplicateAdCampaignsRequest, BuildError> {
        Ok(DuplicateAdCampaignsRequest { count: self.count })
    }
}
