pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrieveTimeSeriesResponseDataPointsItem {
    /// Present only when broken down: one entry per property value in this period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Vec<RetrieveTimeSeriesResponseDataPointsItemBreakdownItem>>,
    /// Present for the funnel metric. People reaching each step in order, starting with the entry step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<FunnelStepResult>>,
    /// Unix timestamp (seconds) of the period start.
    #[serde(default)]
    pub timestamp: i64,
    /// The metric's value for this period, in the metric's unit. For `funnel`, the final-step conversion percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
}

impl RetrieveTimeSeriesResponseDataPointsItem {
    pub fn builder() -> RetrieveTimeSeriesResponseDataPointsItemBuilder {
        <RetrieveTimeSeriesResponseDataPointsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveTimeSeriesResponseDataPointsItemBuilder {
    breakdown: Option<Vec<RetrieveTimeSeriesResponseDataPointsItemBreakdownItem>>,
    steps: Option<Vec<FunnelStepResult>>,
    timestamp: Option<i64>,
    value: Option<f64>,
}

impl RetrieveTimeSeriesResponseDataPointsItemBuilder {
    pub fn breakdown(
        mut self,
        value: Vec<RetrieveTimeSeriesResponseDataPointsItemBreakdownItem>,
    ) -> Self {
        self.breakdown = Some(value);
        self
    }

    pub fn steps(mut self, value: Vec<FunnelStepResult>) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn timestamp(mut self, value: i64) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrieveTimeSeriesResponseDataPointsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timestamp`](RetrieveTimeSeriesResponseDataPointsItemBuilder::timestamp)
    pub fn build(self) -> Result<RetrieveTimeSeriesResponseDataPointsItem, BuildError> {
        Ok(RetrieveTimeSeriesResponseDataPointsItem {
            breakdown: self.breakdown,
            steps: self.steps,
            timestamp: self
                .timestamp
                .ok_or_else(|| BuildError::missing_field("timestamp"))?,
            value: self.value,
        })
    }
}
