pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdStat {
    /// The ID of the ad's campaign, prefixed `adcamp_`.
    #[serde(default)]
    pub ad_campaign_id: String,
    /// The ID of the ad's ad group, prefixed `adgrp_`.
    #[serde(default)]
    pub ad_group_id: String,
    /// The ad's ID, prefixed `ad_`, as sent.
    #[serde(default)]
    pub ad_id: String,
    /// Always 0. Whop stores no clicks for a runner's hour: an ad's clicks are its Whop pixel page views, counted when its stats are shown.
    #[serde(default)]
    pub clicks: i64,
    /// Start of the hour, as an ISO 8601 timestamp.
    #[serde(default)]
    pub hour_started_at: String,
    /// The ad's impressions in the hour.
    #[serde(default)]
    pub impressions: i64,
    /// The spend recorded and paid for the ad in the hour, exactly as reported.
    #[serde(default)]
    pub spend: Money,
}

impl AdStat {
    pub fn builder() -> AdStatBuilder {
        <AdStatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdStatBuilder {
    ad_campaign_id: Option<String>,
    ad_group_id: Option<String>,
    ad_id: Option<String>,
    clicks: Option<i64>,
    hour_started_at: Option<String>,
    impressions: Option<i64>,
    spend: Option<Money>,
}

impl AdStatBuilder {
    pub fn ad_campaign_id(mut self, value: impl Into<String>) -> Self {
        self.ad_campaign_id = Some(value.into());
        self
    }

    pub fn ad_group_id(mut self, value: impl Into<String>) -> Self {
        self.ad_group_id = Some(value.into());
        self
    }

    pub fn ad_id(mut self, value: impl Into<String>) -> Self {
        self.ad_id = Some(value.into());
        self
    }

    pub fn clicks(mut self, value: i64) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn hour_started_at(mut self, value: impl Into<String>) -> Self {
        self.hour_started_at = Some(value.into());
        self
    }

    pub fn impressions(mut self, value: i64) -> Self {
        self.impressions = Some(value);
        self
    }

    pub fn spend(mut self, value: Money) -> Self {
        self.spend = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ad_campaign_id`](AdStatBuilder::ad_campaign_id)
    /// - [`ad_group_id`](AdStatBuilder::ad_group_id)
    /// - [`ad_id`](AdStatBuilder::ad_id)
    /// - [`clicks`](AdStatBuilder::clicks)
    /// - [`hour_started_at`](AdStatBuilder::hour_started_at)
    /// - [`impressions`](AdStatBuilder::impressions)
    /// - [`spend`](AdStatBuilder::spend)
    pub fn build(self) -> Result<AdStat, BuildError> {
        Ok(AdStat {
            ad_campaign_id: self
                .ad_campaign_id
                .ok_or_else(|| BuildError::missing_field("ad_campaign_id"))?,
            ad_group_id: self
                .ad_group_id
                .ok_or_else(|| BuildError::missing_field("ad_group_id"))?,
            ad_id: self
                .ad_id
                .ok_or_else(|| BuildError::missing_field("ad_id"))?,
            clicks: self
                .clicks
                .ok_or_else(|| BuildError::missing_field("clicks"))?,
            hour_started_at: self
                .hour_started_at
                .ok_or_else(|| BuildError::missing_field("hour_started_at"))?,
            impressions: self
                .impressions
                .ok_or_else(|| BuildError::missing_field("impressions"))?,
            spend: self
                .spend
                .ok_or_else(|| BuildError::missing_field("spend"))?,
        })
    }
}
