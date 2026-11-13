pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FunnelConfidenceInterval {
    /// The lower confidence bound, in percent or percentage points for a difference.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub lower: f64,
    /// The upper confidence bound, in percent or percentage points for a difference.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub upper: f64,
}

impl FunnelConfidenceInterval {
    pub fn builder() -> FunnelConfidenceIntervalBuilder {
        <FunnelConfidenceIntervalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FunnelConfidenceIntervalBuilder {
    lower: Option<f64>,
    upper: Option<f64>,
}

impl FunnelConfidenceIntervalBuilder {
    pub fn lower(mut self, value: f64) -> Self {
        self.lower = Some(value);
        self
    }

    pub fn upper(mut self, value: f64) -> Self {
        self.upper = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FunnelConfidenceInterval`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lower`](FunnelConfidenceIntervalBuilder::lower)
    /// - [`upper`](FunnelConfidenceIntervalBuilder::upper)
    pub fn build(self) -> Result<FunnelConfidenceInterval, BuildError> {
        Ok(FunnelConfidenceInterval {
            lower: self
                .lower
                .ok_or_else(|| BuildError::missing_field("lower"))?,
            upper: self
                .upper
                .ok_or_else(|| BuildError::missing_field("upper"))?,
        })
    }
}
