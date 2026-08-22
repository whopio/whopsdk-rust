pub use crate::prelude::*;

/// Per-bucket ad performance for an ad campaign, ad group, or ad. Bucket grain is set by the `ad_report` query's `granularity` argument.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdReportGranularityItem {
    /// The bucket's start time as a real UTC instant. `(statDate, statHour)` resolved in the ad account's reporting timezone — render this in the viewer's local timezone.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub bucket_start: DateTime<FixedOffset>,
    /// Clicks in this bucket.
    #[serde(default)]
    pub clicks: i64,
    /// The bucket size of this row (`hourly`, `daily`, `weekly`, or `monthly`).
    pub granularity: Granularities,
    /// Impressions in this bucket.
    #[serde(default)]
    pub impressions: i64,
    /// Unique users reached in this bucket. Always `0` for hourly rows (Meta does not return reach at hourly grain).
    #[serde(default)]
    pub reach: i64,
    /// Count of the primary optimization result in this bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_count: Option<i64>,
    /// The type of optimization result represented by `resultCount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_label_key: Option<ResultLabelKeys>,
    /// Advertiser-defined label for the result when `resultLabelKey` is `custom`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_label_override: Option<String>,
    /// Charged spend in this bucket in the requested reporting currency — the amount billed including platform fees, not the platform-side net spend.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub spend: f64,
    /// Currency of the `spend` value.
    pub spend_currency: Currencies,
    /// The date these stats cover (midnight UTC). For hourly rows, see `statHour` and `bucketStart`.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub stat_date: DateTime<FixedOffset>,
    /// Hour of the day in the ad account's reporting timezone (0-23). `null` for daily rows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stat_hour: Option<i64>,
}

impl AdReportGranularityItem {
    pub fn builder() -> AdReportGranularityItemBuilder {
        <AdReportGranularityItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdReportGranularityItemBuilder {
    bucket_start: Option<DateTime<FixedOffset>>,
    clicks: Option<i64>,
    granularity: Option<Granularities>,
    impressions: Option<i64>,
    reach: Option<i64>,
    result_count: Option<i64>,
    result_label_key: Option<ResultLabelKeys>,
    result_label_override: Option<String>,
    spend: Option<f64>,
    spend_currency: Option<Currencies>,
    stat_date: Option<DateTime<FixedOffset>>,
    stat_hour: Option<i64>,
}

impl AdReportGranularityItemBuilder {
    pub fn bucket_start(mut self, value: DateTime<FixedOffset>) -> Self {
        self.bucket_start = Some(value);
        self
    }

    pub fn clicks(mut self, value: i64) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn granularity(mut self, value: Granularities) -> Self {
        self.granularity = Some(value);
        self
    }

    pub fn impressions(mut self, value: i64) -> Self {
        self.impressions = Some(value);
        self
    }

    pub fn reach(mut self, value: i64) -> Self {
        self.reach = Some(value);
        self
    }

    pub fn result_count(mut self, value: i64) -> Self {
        self.result_count = Some(value);
        self
    }

    pub fn result_label_key(mut self, value: ResultLabelKeys) -> Self {
        self.result_label_key = Some(value);
        self
    }

    pub fn result_label_override(mut self, value: impl Into<String>) -> Self {
        self.result_label_override = Some(value.into());
        self
    }

    pub fn spend(mut self, value: f64) -> Self {
        self.spend = Some(value);
        self
    }

    pub fn spend_currency(mut self, value: Currencies) -> Self {
        self.spend_currency = Some(value);
        self
    }

    pub fn stat_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.stat_date = Some(value);
        self
    }

    pub fn stat_hour(mut self, value: i64) -> Self {
        self.stat_hour = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdReportGranularityItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bucket_start`](AdReportGranularityItemBuilder::bucket_start)
    /// - [`clicks`](AdReportGranularityItemBuilder::clicks)
    /// - [`granularity`](AdReportGranularityItemBuilder::granularity)
    /// - [`impressions`](AdReportGranularityItemBuilder::impressions)
    /// - [`reach`](AdReportGranularityItemBuilder::reach)
    /// - [`spend`](AdReportGranularityItemBuilder::spend)
    /// - [`spend_currency`](AdReportGranularityItemBuilder::spend_currency)
    /// - [`stat_date`](AdReportGranularityItemBuilder::stat_date)
    pub fn build(self) -> Result<AdReportGranularityItem, BuildError> {
        Ok(AdReportGranularityItem {
            bucket_start: self
                .bucket_start
                .ok_or_else(|| BuildError::missing_field("bucket_start"))?,
            clicks: self
                .clicks
                .ok_or_else(|| BuildError::missing_field("clicks"))?,
            granularity: self
                .granularity
                .ok_or_else(|| BuildError::missing_field("granularity"))?,
            impressions: self
                .impressions
                .ok_or_else(|| BuildError::missing_field("impressions"))?,
            reach: self
                .reach
                .ok_or_else(|| BuildError::missing_field("reach"))?,
            result_count: self.result_count,
            result_label_key: self.result_label_key,
            result_label_override: self.result_label_override,
            spend: self
                .spend
                .ok_or_else(|| BuildError::missing_field("spend"))?,
            spend_currency: self
                .spend_currency
                .ok_or_else(|| BuildError::missing_field("spend_currency"))?,
            stat_date: self
                .stat_date
                .ok_or_else(|| BuildError::missing_field("stat_date"))?,
            stat_hour: self.stat_hour,
        })
    }
}
