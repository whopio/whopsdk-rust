pub use crate::prelude::*;

/// Aggregate totals and rates for this entity over the date range.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdReportBreakdownItemSummary {
    /// Click-through rate (clicks / impressions).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub click_through_rate: f64,
    /// Total clicks over the date range.
    #[serde(default)]
    pub clicks: i64,
    /// Cost per click in the requested reporting currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub cost_per_click: f64,
    /// Cost per thousand impressions in the requested reporting currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_per_mille: Option<f64>,
    /// Spend divided by `resultCount`. Null when there are no results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_per_result: Option<f64>,
    /// Average number of times each reached user saw an ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub frequency: Option<f64>,
    /// Total impressions over the date range.
    #[serde(default)]
    pub impressions: i64,
    /// Unique users reached, deduplicated by the external ad platform.
    #[serde(default)]
    pub reach: i64,
    /// Count of the campaign's primary optimization result (purchases, clicks, etc.) — see `resultLabelKey`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_count: Option<i64>,
    /// The type of optimization result represented by `resultCount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_label_key: Option<ResultLabelKeys>,
    /// Advertiser-defined label for the result when `resultLabelKey` is `custom`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_label_override: Option<String>,
    /// Alias for `purchaseReturnOnAdSpend` — return on ad spend for purchases, as reported by the external ad platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub return_on_ad_spend: Option<f64>,
    /// Total spend over the date range in the requested reporting currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub spend: f64,
    /// Currency of the `spend` value. Matches the requested `currency` when set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_currency: Option<Currencies>,
}

impl AdReportBreakdownItemSummary {
    pub fn builder() -> AdReportBreakdownItemSummaryBuilder {
        <AdReportBreakdownItemSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdReportBreakdownItemSummaryBuilder {
    click_through_rate: Option<f64>,
    clicks: Option<i64>,
    cost_per_click: Option<f64>,
    cost_per_mille: Option<f64>,
    cost_per_result: Option<f64>,
    frequency: Option<f64>,
    impressions: Option<i64>,
    reach: Option<i64>,
    result_count: Option<i64>,
    result_label_key: Option<ResultLabelKeys>,
    result_label_override: Option<String>,
    return_on_ad_spend: Option<f64>,
    spend: Option<f64>,
    spend_currency: Option<Currencies>,
}

impl AdReportBreakdownItemSummaryBuilder {
    pub fn click_through_rate(mut self, value: f64) -> Self {
        self.click_through_rate = Some(value);
        self
    }

    pub fn clicks(mut self, value: i64) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn cost_per_click(mut self, value: f64) -> Self {
        self.cost_per_click = Some(value);
        self
    }

    pub fn cost_per_mille(mut self, value: f64) -> Self {
        self.cost_per_mille = Some(value);
        self
    }

    pub fn cost_per_result(mut self, value: f64) -> Self {
        self.cost_per_result = Some(value);
        self
    }

    pub fn frequency(mut self, value: f64) -> Self {
        self.frequency = Some(value);
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

    pub fn return_on_ad_spend(mut self, value: f64) -> Self {
        self.return_on_ad_spend = Some(value);
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

    /// Consumes the builder and constructs a [`AdReportBreakdownItemSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`click_through_rate`](AdReportBreakdownItemSummaryBuilder::click_through_rate)
    /// - [`clicks`](AdReportBreakdownItemSummaryBuilder::clicks)
    /// - [`cost_per_click`](AdReportBreakdownItemSummaryBuilder::cost_per_click)
    /// - [`impressions`](AdReportBreakdownItemSummaryBuilder::impressions)
    /// - [`reach`](AdReportBreakdownItemSummaryBuilder::reach)
    /// - [`spend`](AdReportBreakdownItemSummaryBuilder::spend)
    pub fn build(self) -> Result<AdReportBreakdownItemSummary, BuildError> {
        Ok(AdReportBreakdownItemSummary {
            click_through_rate: self
                .click_through_rate
                .ok_or_else(|| BuildError::missing_field("click_through_rate"))?,
            clicks: self
                .clicks
                .ok_or_else(|| BuildError::missing_field("clicks"))?,
            cost_per_click: self
                .cost_per_click
                .ok_or_else(|| BuildError::missing_field("cost_per_click"))?,
            cost_per_mille: self.cost_per_mille,
            cost_per_result: self.cost_per_result,
            frequency: self.frequency,
            impressions: self
                .impressions
                .ok_or_else(|| BuildError::missing_field("impressions"))?,
            reach: self
                .reach
                .ok_or_else(|| BuildError::missing_field("reach"))?,
            result_count: self.result_count,
            result_label_key: self.result_label_key,
            result_label_override: self.result_label_override,
            return_on_ad_spend: self.return_on_ad_spend,
            spend: self
                .spend
                .ok_or_else(|| BuildError::missing_field("spend"))?,
            spend_currency: self.spend_currency,
        })
    }
}
