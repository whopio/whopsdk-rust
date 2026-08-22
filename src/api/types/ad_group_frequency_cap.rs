pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdGroupFrequencyCap {
    /// Most times one person can be shown ads from this ad group within the window.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub maximum_impressions: f64,
    /// Length of the rolling window, in days.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub per_days: Option<f64>,
}

impl AdGroupFrequencyCap {
    pub fn builder() -> AdGroupFrequencyCapBuilder {
        <AdGroupFrequencyCapBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupFrequencyCapBuilder {
    maximum_impressions: Option<f64>,
    per_days: Option<f64>,
}

impl AdGroupFrequencyCapBuilder {
    pub fn maximum_impressions(mut self, value: f64) -> Self {
        self.maximum_impressions = Some(value);
        self
    }

    pub fn per_days(mut self, value: f64) -> Self {
        self.per_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupFrequencyCap`].
    /// This method will fail if any of the following fields are not set:
    /// - [`maximum_impressions`](AdGroupFrequencyCapBuilder::maximum_impressions)
    pub fn build(self) -> Result<AdGroupFrequencyCap, BuildError> {
        Ok(AdGroupFrequencyCap {
            maximum_impressions: self
                .maximum_impressions
                .ok_or_else(|| BuildError::missing_field("maximum_impressions"))?,
            per_days: self.per_days,
        })
    }
}
