pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdGroupLearningProgress {
    /// The conversions this ad group's learning phase needs. It can change during delivery.
    #[serde(default)]
    pub conversion_threshold: i64,
    /// Conversions counted since the last significant edit during this ad group's learning phase.
    #[serde(default)]
    pub conversions: i64,
    /// Conversions divided by the threshold, as a percentage clamped to 0–100. The ad group's `delivery_status` remains authoritative.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub progress_percent: f64,
}

impl AdGroupLearningProgress {
    pub fn builder() -> AdGroupLearningProgressBuilder {
        <AdGroupLearningProgressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupLearningProgressBuilder {
    conversion_threshold: Option<i64>,
    conversions: Option<i64>,
    progress_percent: Option<f64>,
}

impl AdGroupLearningProgressBuilder {
    pub fn conversion_threshold(mut self, value: i64) -> Self {
        self.conversion_threshold = Some(value);
        self
    }

    pub fn conversions(mut self, value: i64) -> Self {
        self.conversions = Some(value);
        self
    }

    pub fn progress_percent(mut self, value: f64) -> Self {
        self.progress_percent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupLearningProgress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversion_threshold`](AdGroupLearningProgressBuilder::conversion_threshold)
    /// - [`conversions`](AdGroupLearningProgressBuilder::conversions)
    /// - [`progress_percent`](AdGroupLearningProgressBuilder::progress_percent)
    pub fn build(self) -> Result<AdGroupLearningProgress, BuildError> {
        Ok(AdGroupLearningProgress {
            conversion_threshold: self
                .conversion_threshold
                .ok_or_else(|| BuildError::missing_field("conversion_threshold"))?,
            conversions: self
                .conversions
                .ok_or_else(|| BuildError::missing_field("conversions"))?,
            progress_percent: self
                .progress_percent
                .ok_or_else(|| BuildError::missing_field("progress_percent"))?,
        })
    }
}
