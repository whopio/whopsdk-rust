pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DuplicateAdsRequest {
    /// Number of copies to create (1-10). Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Ad group to duplicate into, which must belong to the same account and be compatible with the ad. Defaults to the ad's own ad group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ad_group_id: Option<String>,
}

impl DuplicateAdsRequest {
    pub fn builder() -> DuplicateAdsRequestBuilder {
        <DuplicateAdsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DuplicateAdsRequestBuilder {
    count: Option<i64>,
    target_ad_group_id: Option<String>,
}

impl DuplicateAdsRequestBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn target_ad_group_id(mut self, value: impl Into<String>) -> Self {
        self.target_ad_group_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DuplicateAdsRequest`].
    pub fn build(self) -> Result<DuplicateAdsRequest, BuildError> {
        Ok(DuplicateAdsRequest {
            count: self.count,
            target_ad_group_id: self.target_ad_group_id,
        })
    }
}
