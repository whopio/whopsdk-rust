pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FunnelStepResult {
    /// The percentage of first-step entrants reaching this step; null when there are no entrants.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub conversion_rate: Option<f64>,
    /// The number of distinct people reaching this step.
    #[serde(default)]
    pub count: i64,
    /// The one-based step number.
    #[serde(default)]
    pub step: i64,
}

impl FunnelStepResult {
    pub fn builder() -> FunnelStepResultBuilder {
        <FunnelStepResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FunnelStepResultBuilder {
    conversion_rate: Option<f64>,
    count: Option<i64>,
    step: Option<i64>,
}

impl FunnelStepResultBuilder {
    pub fn conversion_rate(mut self, value: f64) -> Self {
        self.conversion_rate = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn step(mut self, value: i64) -> Self {
        self.step = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FunnelStepResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](FunnelStepResultBuilder::count)
    /// - [`step`](FunnelStepResultBuilder::step)
    pub fn build(self) -> Result<FunnelStepResult, BuildError> {
        Ok(FunnelStepResult {
            conversion_rate: self.conversion_rate,
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            step: self.step.ok_or_else(|| BuildError::missing_field("step"))?,
        })
    }
}
