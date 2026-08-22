pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DuplicateAdGroupsRequest {
    /// Number of copies to create (1-10). Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Whether the copied ads keep the original posts' engagement (likes, comments, shares). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_engagement: Option<bool>,
    /// Campaign to duplicate into. Defaults to the ad group's own campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ad_campaign_id: Option<String>,
}

impl DuplicateAdGroupsRequest {
    pub fn builder() -> DuplicateAdGroupsRequestBuilder {
        <DuplicateAdGroupsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DuplicateAdGroupsRequestBuilder {
    count: Option<i64>,
    preserve_engagement: Option<bool>,
    target_ad_campaign_id: Option<String>,
}

impl DuplicateAdGroupsRequestBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn preserve_engagement(mut self, value: bool) -> Self {
        self.preserve_engagement = Some(value);
        self
    }

    pub fn target_ad_campaign_id(mut self, value: impl Into<String>) -> Self {
        self.target_ad_campaign_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DuplicateAdGroupsRequest`].
    pub fn build(self) -> Result<DuplicateAdGroupsRequest, BuildError> {
        Ok(DuplicateAdGroupsRequest {
            count: self.count,
            preserve_engagement: self.preserve_engagement,
            target_ad_campaign_id: self.target_ad_campaign_id,
        })
    }
}
