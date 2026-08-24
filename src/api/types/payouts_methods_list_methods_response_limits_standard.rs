pub use crate::prelude::*;

/// Caps for standard-speed payouts, which draw on settled funds only.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMethodsResponseLimitsStandard {
    /// The maximum amount a standard payout can move right now, in whole currency units.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub max_amount: f64,
}

impl ListMethodsResponseLimitsStandard {
    pub fn builder() -> ListMethodsResponseLimitsStandardBuilder {
        <ListMethodsResponseLimitsStandardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMethodsResponseLimitsStandardBuilder {
    max_amount: Option<f64>,
}

impl ListMethodsResponseLimitsStandardBuilder {
    pub fn max_amount(mut self, value: f64) -> Self {
        self.max_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMethodsResponseLimitsStandard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`max_amount`](ListMethodsResponseLimitsStandardBuilder::max_amount)
    pub fn build(self) -> Result<ListMethodsResponseLimitsStandard, BuildError> {
        Ok(ListMethodsResponseLimitsStandard {
            max_amount: self
                .max_amount
                .ok_or_else(|| BuildError::missing_field("max_amount"))?,
        })
    }
}
