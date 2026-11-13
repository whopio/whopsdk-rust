pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FunnelComparison {
    /// The reference group's name.
    #[serde(default)]
    pub baseline: String,
    /// Adjusted Wald interval for the difference in percentage points, with Bonferroni correction across comparisons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difference_confidence_interval: Option<FunnelConfidenceInterval>,
    /// This group's conversion percentage minus the baseline's.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub difference_percentage_points: f64,
    /// Relative conversion improvement in percent; null if baseline conversion is zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub relative_lift_percent: Option<f64>,
}

impl FunnelComparison {
    pub fn builder() -> FunnelComparisonBuilder {
        <FunnelComparisonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FunnelComparisonBuilder {
    baseline: Option<String>,
    difference_confidence_interval: Option<FunnelConfidenceInterval>,
    difference_percentage_points: Option<f64>,
    relative_lift_percent: Option<f64>,
}

impl FunnelComparisonBuilder {
    pub fn baseline(mut self, value: impl Into<String>) -> Self {
        self.baseline = Some(value.into());
        self
    }

    pub fn difference_confidence_interval(mut self, value: FunnelConfidenceInterval) -> Self {
        self.difference_confidence_interval = Some(value);
        self
    }

    pub fn difference_percentage_points(mut self, value: f64) -> Self {
        self.difference_percentage_points = Some(value);
        self
    }

    pub fn relative_lift_percent(mut self, value: f64) -> Self {
        self.relative_lift_percent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FunnelComparison`].
    /// This method will fail if any of the following fields are not set:
    /// - [`baseline`](FunnelComparisonBuilder::baseline)
    /// - [`difference_percentage_points`](FunnelComparisonBuilder::difference_percentage_points)
    pub fn build(self) -> Result<FunnelComparison, BuildError> {
        Ok(FunnelComparison {
            baseline: self
                .baseline
                .ok_or_else(|| BuildError::missing_field("baseline"))?,
            difference_confidence_interval: self.difference_confidence_interval,
            difference_percentage_points: self
                .difference_percentage_points
                .ok_or_else(|| BuildError::missing_field("difference_percentage_points"))?,
            relative_lift_percent: self.relative_lift_percent,
        })
    }
}
