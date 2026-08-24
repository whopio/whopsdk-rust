pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReachEstimate {
    /// Low end of how many people the targeting can reach. Null when the platform couldn't produce an estimate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub users_lower_bound: Option<f64>,
    /// High end of how many people the targeting can reach. Null when the platform couldn't produce an estimate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub users_upper_bound: Option<f64>,
}

impl ReachEstimate {
    pub fn builder() -> ReachEstimateBuilder {
        <ReachEstimateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReachEstimateBuilder {
    users_lower_bound: Option<f64>,
    users_upper_bound: Option<f64>,
}

impl ReachEstimateBuilder {
    pub fn users_lower_bound(mut self, value: f64) -> Self {
        self.users_lower_bound = Some(value);
        self
    }

    pub fn users_upper_bound(mut self, value: f64) -> Self {
        self.users_upper_bound = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReachEstimate`].
    pub fn build(self) -> Result<ReachEstimate, BuildError> {
        Ok(ReachEstimate {
            users_lower_bound: self.users_lower_bound,
            users_upper_bound: self.users_upper_bound,
        })
    }
}
