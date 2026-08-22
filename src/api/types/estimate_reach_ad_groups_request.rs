pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EstimateReachAdGroupsRequest {
    /// Account to estimate on behalf of. Defaults to the authenticated account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Saved audiences to deliver to or exclude. Can't be combined with demographics.automatic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiences: Option<AdGroupAudiencesBody>,
    /// Age, gender, and automatic-audience targeting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographics: Option<AdGroupDemographicsBody>,
    /// Interest, behavior, and demographic targeting, using categories from the ad platform's targeting taxonomy. At most 100 entries per section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_targeting: Option<AdGroupDetailedTargetingBody>,
    /// Device platforms and operating systems to target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<AdGroupDevicesBody>,
    /// Languages to target, as ISO 639 codes such as `en` or `es`. Empty or omitted targets all languages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    /// The ad network the estimate runs on.
    pub platform: EstimateReachAdGroupsRequestPlatform,
    /// Locations to target and exclude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<AdGroupRegionsBody>,
}

impl EstimateReachAdGroupsRequest {
    pub fn builder() -> EstimateReachAdGroupsRequestBuilder {
        <EstimateReachAdGroupsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EstimateReachAdGroupsRequestBuilder {
    account_id: Option<String>,
    audiences: Option<AdGroupAudiencesBody>,
    demographics: Option<AdGroupDemographicsBody>,
    detailed_targeting: Option<AdGroupDetailedTargetingBody>,
    devices: Option<AdGroupDevicesBody>,
    languages: Option<Vec<String>>,
    platform: Option<EstimateReachAdGroupsRequestPlatform>,
    regions: Option<AdGroupRegionsBody>,
}

impl EstimateReachAdGroupsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn audiences(mut self, value: AdGroupAudiencesBody) -> Self {
        self.audiences = Some(value);
        self
    }

    pub fn demographics(mut self, value: AdGroupDemographicsBody) -> Self {
        self.demographics = Some(value);
        self
    }

    pub fn detailed_targeting(mut self, value: AdGroupDetailedTargetingBody) -> Self {
        self.detailed_targeting = Some(value);
        self
    }

    pub fn devices(mut self, value: AdGroupDevicesBody) -> Self {
        self.devices = Some(value);
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn platform(mut self, value: EstimateReachAdGroupsRequestPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn regions(mut self, value: AdGroupRegionsBody) -> Self {
        self.regions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EstimateReachAdGroupsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform`](EstimateReachAdGroupsRequestBuilder::platform)
    pub fn build(self) -> Result<EstimateReachAdGroupsRequest, BuildError> {
        Ok(EstimateReachAdGroupsRequest {
            account_id: self.account_id,
            audiences: self.audiences,
            demographics: self.demographics,
            detailed_targeting: self.detailed_targeting,
            devices: self.devices,
            languages: self.languages,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            regions: self.regions,
        })
    }
}
