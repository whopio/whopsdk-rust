pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrieveTimeSeriesResponseDataTotalsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<FunnelComparison>,
    /// Funnel only. The Wilson interval for final conversion, present when confidence_level or compare_to is supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_interval: Option<FunnelConfidenceInterval>,
    /// The property value this total is for, or the metric's name when it isn't split by a property.
    #[serde(default)]
    pub name: String,
    /// Present for the funnel metric. People reaching each step in order, starting with the entry step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<FunnelStepResult>>,
    /// The metric's whole-window value for this entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
}

impl RetrieveTimeSeriesResponseDataTotalsItem {
    pub fn builder() -> RetrieveTimeSeriesResponseDataTotalsItemBuilder {
        <RetrieveTimeSeriesResponseDataTotalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveTimeSeriesResponseDataTotalsItemBuilder {
    comparison: Option<FunnelComparison>,
    confidence_interval: Option<FunnelConfidenceInterval>,
    name: Option<String>,
    steps: Option<Vec<FunnelStepResult>>,
    value: Option<f64>,
}

impl RetrieveTimeSeriesResponseDataTotalsItemBuilder {
    pub fn comparison(mut self, value: FunnelComparison) -> Self {
        self.comparison = Some(value);
        self
    }

    pub fn confidence_interval(mut self, value: FunnelConfidenceInterval) -> Self {
        self.confidence_interval = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn steps(mut self, value: Vec<FunnelStepResult>) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrieveTimeSeriesResponseDataTotalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RetrieveTimeSeriesResponseDataTotalsItemBuilder::name)
    pub fn build(self) -> Result<RetrieveTimeSeriesResponseDataTotalsItem, BuildError> {
        Ok(RetrieveTimeSeriesResponseDataTotalsItem {
            comparison: self.comparison,
            confidence_interval: self.confidence_interval,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            steps: self.steps,
            value: self.value,
        })
    }
}
