pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportIssuesAdsRequest {
    /// The ID of the ad group the issues are on, prefixed `adgrp_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_group_id: Option<String>,
    /// The ID of the ad the issues are on, prefixed `ad_`. Send exactly one of `ad_id` or `ad_group_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_id: Option<String>,
    /// Every issue the object has now, each reason once. Send an empty list when it has none.
    #[serde(default)]
    pub issues: Vec<ReportIssuesAdsRequestIssuesItem>,
}

impl ReportIssuesAdsRequest {
    pub fn builder() -> ReportIssuesAdsRequestBuilder {
        <ReportIssuesAdsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportIssuesAdsRequestBuilder {
    ad_group_id: Option<String>,
    ad_id: Option<String>,
    issues: Option<Vec<ReportIssuesAdsRequestIssuesItem>>,
}

impl ReportIssuesAdsRequestBuilder {
    pub fn ad_group_id(mut self, value: impl Into<String>) -> Self {
        self.ad_group_id = Some(value.into());
        self
    }

    pub fn ad_id(mut self, value: impl Into<String>) -> Self {
        self.ad_id = Some(value.into());
        self
    }

    pub fn issues(mut self, value: Vec<ReportIssuesAdsRequestIssuesItem>) -> Self {
        self.issues = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportIssuesAdsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`issues`](ReportIssuesAdsRequestBuilder::issues)
    pub fn build(self) -> Result<ReportIssuesAdsRequest, BuildError> {
        Ok(ReportIssuesAdsRequest {
            ad_group_id: self.ad_group_id,
            ad_id: self.ad_id,
            issues: self
                .issues
                .ok_or_else(|| BuildError::missing_field("issues"))?,
        })
    }
}
