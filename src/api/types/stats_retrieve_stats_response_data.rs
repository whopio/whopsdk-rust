pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrieveStatsResponseData {
    /// Present for webhook_deliveries. False while older deliveries are still being counted in the background: the points cover what is counted so far. Responses are cached for up to a minute, so wait a minute before requesting the same range again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// ISO currency the values are denominated in. Present for currency-unit metrics: the convert_to currency, or usd.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// One entry per period, oldest first.
    #[serde(default)]
    pub points: Vec<RetrieveStatsResponseDataPointsItem>,
    /// Whole-window aggregates, present when the metric computes them. Don't derive these from `points`: a rate is measured across the whole window, not averaged across its points, and whole-window unique people are distinct across every bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals: Option<Vec<RetrieveStatsResponseDataTotalsItem>>,
}

impl RetrieveStatsResponseData {
    pub fn builder() -> RetrieveStatsResponseDataBuilder {
        <RetrieveStatsResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveStatsResponseDataBuilder {
    complete: Option<bool>,
    currency: Option<String>,
    points: Option<Vec<RetrieveStatsResponseDataPointsItem>>,
    totals: Option<Vec<RetrieveStatsResponseDataTotalsItem>>,
}

impl RetrieveStatsResponseDataBuilder {
    pub fn complete(mut self, value: bool) -> Self {
        self.complete = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn points(mut self, value: Vec<RetrieveStatsResponseDataPointsItem>) -> Self {
        self.points = Some(value);
        self
    }

    pub fn totals(mut self, value: Vec<RetrieveStatsResponseDataTotalsItem>) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrieveStatsResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`points`](RetrieveStatsResponseDataBuilder::points)
    pub fn build(self) -> Result<RetrieveStatsResponseData, BuildError> {
        Ok(RetrieveStatsResponseData {
            complete: self.complete,
            currency: self.currency,
            points: self
                .points
                .ok_or_else(|| BuildError::missing_field("points"))?,
            totals: self.totals,
        })
    }
}
