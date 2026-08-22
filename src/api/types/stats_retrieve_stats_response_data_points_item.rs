pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrieveStatsResponseDataPointsItem {
    /// Present only when broken down: one entry per property value in this period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Vec<RetrieveStatsResponseDataPointsItemBreakdownItem>>,
    /// Unix timestamp (seconds) of the period start.
    #[serde(default)]
    pub timestamp: i64,
    /// The metric's value for this period, in the metric's unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
}

impl RetrieveStatsResponseDataPointsItem {
    pub fn builder() -> RetrieveStatsResponseDataPointsItemBuilder {
        <RetrieveStatsResponseDataPointsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveStatsResponseDataPointsItemBuilder {
    breakdown: Option<Vec<RetrieveStatsResponseDataPointsItemBreakdownItem>>,
    timestamp: Option<i64>,
    value: Option<f64>,
}

impl RetrieveStatsResponseDataPointsItemBuilder {
    pub fn breakdown(
        mut self,
        value: Vec<RetrieveStatsResponseDataPointsItemBreakdownItem>,
    ) -> Self {
        self.breakdown = Some(value);
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

    /// Consumes the builder and constructs a [`RetrieveStatsResponseDataPointsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timestamp`](RetrieveStatsResponseDataPointsItemBuilder::timestamp)
    pub fn build(self) -> Result<RetrieveStatsResponseDataPointsItem, BuildError> {
        Ok(RetrieveStatsResponseDataPointsItem {
            breakdown: self.breakdown,
            timestamp: self
                .timestamp
                .ok_or_else(|| BuildError::missing_field("timestamp"))?,
            value: self.value,
        })
    }
}
