pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrieveStatsResponseData {
    /// ISO currency the values are denominated in. Present for currency-unit metrics: the convert_to currency, or usd.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// One entry per period, oldest first.
    #[serde(default)]
    pub points: Vec<RetrieveStatsResponseDataPointsItem>,
    /// Whole-window aggregates, present when the metric computes them. Don't derive these from `points`: a rate is measured across the whole window, not averaged across its points, and unique-people counts exist only at window level.
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
    currency: Option<String>,
    points: Option<Vec<RetrieveStatsResponseDataPointsItem>>,
    totals: Option<Vec<RetrieveStatsResponseDataTotalsItem>>,
}

impl RetrieveStatsResponseDataBuilder {
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
            currency: self.currency,
            points: self
                .points
                .ok_or_else(|| BuildError::missing_field("points"))?,
            totals: self.totals,
        })
    }
}
