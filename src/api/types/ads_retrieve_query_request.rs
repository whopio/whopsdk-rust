pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdsRetrieveQueryRequest {
    /// Start of the stats window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_from: Option<String>,
    /// End of the stats window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_to: Option<String>,
    /// IANA timezone the stats window is interpreted in. Defaults to UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Attribution model the conversion stats count under (defaults to last_touch). Under both models a journey with any whop ad touch attributes to whop; the model picks which whop touch credits the entity and which non-whop source wins otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution_model: Option<RetrieveAdsRequestAttributionModel>,
}

impl AdsRetrieveQueryRequest {
    pub fn builder() -> AdsRetrieveQueryRequestBuilder {
        <AdsRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdsRetrieveQueryRequestBuilder {
    stats_from: Option<String>,
    stats_to: Option<String>,
    time_zone: Option<String>,
    attribution_model: Option<RetrieveAdsRequestAttributionModel>,
}

impl AdsRetrieveQueryRequestBuilder {
    pub fn stats_from(mut self, value: impl Into<String>) -> Self {
        self.stats_from = Some(value.into());
        self
    }

    pub fn stats_to(mut self, value: impl Into<String>) -> Self {
        self.stats_to = Some(value.into());
        self
    }

    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }

    pub fn attribution_model(mut self, value: RetrieveAdsRequestAttributionModel) -> Self {
        self.attribution_model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdsRetrieveQueryRequest`].
    pub fn build(self) -> Result<AdsRetrieveQueryRequest, BuildError> {
        Ok(AdsRetrieveQueryRequest {
            stats_from: self.stats_from,
            stats_to: self.stats_to,
            time_zone: self.time_zone,
            attribution_model: self.attribution_model,
        })
    }
}
