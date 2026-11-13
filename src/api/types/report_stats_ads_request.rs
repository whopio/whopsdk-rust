pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReportStatsAdsRequest {
    /// The ad's ID, prefixed `ad_`: the campaign's ad or the runner's copy of it.
    #[serde(default)]
    pub ad_id: String,
    /// Start of the hour, on the hour, for an hour that has already started.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub hour_started_at: DateTime<FixedOffset>,
    /// The ad's impressions in the hour so far. Zero when left out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impressions: Option<ReportStatsAdsRequestImpressions>,
    /// The USD to be paid for the ad's hour so far, to the cent: what the ad spent plus any markup, such as 105.00 for 100.00 spent with a 5% markup.
    pub spend: ReportStatsAdsRequestSpend,
}

impl ReportStatsAdsRequest {
    pub fn builder() -> ReportStatsAdsRequestBuilder {
        <ReportStatsAdsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportStatsAdsRequestBuilder {
    ad_id: Option<String>,
    hour_started_at: Option<DateTime<FixedOffset>>,
    impressions: Option<ReportStatsAdsRequestImpressions>,
    spend: Option<ReportStatsAdsRequestSpend>,
}

impl ReportStatsAdsRequestBuilder {
    pub fn ad_id(mut self, value: impl Into<String>) -> Self {
        self.ad_id = Some(value.into());
        self
    }

    pub fn hour_started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.hour_started_at = Some(value);
        self
    }

    pub fn impressions(mut self, value: ReportStatsAdsRequestImpressions) -> Self {
        self.impressions = Some(value);
        self
    }

    pub fn spend(mut self, value: ReportStatsAdsRequestSpend) -> Self {
        self.spend = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportStatsAdsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ad_id`](ReportStatsAdsRequestBuilder::ad_id)
    /// - [`hour_started_at`](ReportStatsAdsRequestBuilder::hour_started_at)
    /// - [`spend`](ReportStatsAdsRequestBuilder::spend)
    pub fn build(self) -> Result<ReportStatsAdsRequest, BuildError> {
        Ok(ReportStatsAdsRequest {
            ad_id: self
                .ad_id
                .ok_or_else(|| BuildError::missing_field("ad_id"))?,
            hour_started_at: self
                .hour_started_at
                .ok_or_else(|| BuildError::missing_field("hour_started_at"))?,
            impressions: self.impressions,
            spend: self
                .spend
                .ok_or_else(|| BuildError::missing_field("spend"))?,
        })
    }
}
