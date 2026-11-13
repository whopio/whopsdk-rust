pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrieveTimeSeriesResponseDataPointsItemBreakdownItem {
    /// The property value, for example usd or visa.
    #[serde(default)]
    pub name: String,
    /// Present for the funnel metric. People reaching each step in order, starting with the entry step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<FunnelStepResult>>,
    /// The metric's value for this entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
}

impl RetrieveTimeSeriesResponseDataPointsItemBreakdownItem {
    pub fn builder() -> RetrieveTimeSeriesResponseDataPointsItemBreakdownItemBuilder {
        <RetrieveTimeSeriesResponseDataPointsItemBreakdownItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveTimeSeriesResponseDataPointsItemBreakdownItemBuilder {
    name: Option<String>,
    steps: Option<Vec<FunnelStepResult>>,
    value: Option<f64>,
}

impl RetrieveTimeSeriesResponseDataPointsItemBreakdownItemBuilder {
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

    /// Consumes the builder and constructs a [`RetrieveTimeSeriesResponseDataPointsItemBreakdownItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RetrieveTimeSeriesResponseDataPointsItemBreakdownItemBuilder::name)
    pub fn build(
        self,
    ) -> Result<RetrieveTimeSeriesResponseDataPointsItemBreakdownItem, BuildError> {
        Ok(RetrieveTimeSeriesResponseDataPointsItemBreakdownItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            steps: self.steps,
            value: self.value,
        })
    }
}
