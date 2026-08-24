pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudienceMatchRate {
    /// Lower bound of the estimated match rate percentage. `null` until available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lower_bound: Option<f64>,
    /// The ad platform that provided the match-rate estimate.
    pub platform: AudienceMatchRatePlatform,
    /// Availability of the estimated match rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AudienceMatchRateStatus>,
    /// Upper bound of the estimated match rate percentage. `null` until available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub upper_bound: Option<f64>,
}

impl AudienceMatchRate {
    pub fn builder() -> AudienceMatchRateBuilder {
        <AudienceMatchRateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudienceMatchRateBuilder {
    lower_bound: Option<f64>,
    platform: Option<AudienceMatchRatePlatform>,
    status: Option<AudienceMatchRateStatus>,
    upper_bound: Option<f64>,
}

impl AudienceMatchRateBuilder {
    pub fn lower_bound(mut self, value: f64) -> Self {
        self.lower_bound = Some(value);
        self
    }

    pub fn platform(mut self, value: AudienceMatchRatePlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn status(mut self, value: AudienceMatchRateStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn upper_bound(mut self, value: f64) -> Self {
        self.upper_bound = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudienceMatchRate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform`](AudienceMatchRateBuilder::platform)
    pub fn build(self) -> Result<AudienceMatchRate, BuildError> {
        Ok(AudienceMatchRate {
            lower_bound: self.lower_bound,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            status: self.status,
            upper_bound: self.upper_bound,
        })
    }
}
