pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DuplicateAdCampaignsRequest {
    /// Number of copies to create (1-10). Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Whether the copied ads keep the original posts' engagement (likes, comments, shares). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_engagement: Option<bool>,
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
    preserve_engagement: Option<bool>,
}

impl DuplicateAdCampaignsRequestBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn preserve_engagement(mut self, value: bool) -> Self {
        self.preserve_engagement = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DuplicateAdCampaignsRequest`].
    pub fn build(self) -> Result<DuplicateAdCampaignsRequest, BuildError> {
        Ok(DuplicateAdCampaignsRequest {
            count: self.count,
            preserve_engagement: self.preserve_engagement,
        })
    }
}
