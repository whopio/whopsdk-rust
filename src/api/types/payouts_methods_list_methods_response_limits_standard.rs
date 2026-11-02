pub use crate::prelude::*;

/// Caps for standard-speed payouts, which draw on settled funds only.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMethodsResponseLimitsStandard {
    /// Why a standard payout cannot move funds right now, or null when the cap is above 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<ListMethodsResponseLimitsStandardErrorCode>,
    /// Human-readable form of error_code, or null when a standard payout can move funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
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
    error_code: Option<ListMethodsResponseLimitsStandardErrorCode>,
    error_message: Option<String>,
    max_amount: Option<f64>,
}

impl ListMethodsResponseLimitsStandardBuilder {
    pub fn error_code(mut self, value: ListMethodsResponseLimitsStandardErrorCode) -> Self {
        self.error_code = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn max_amount(mut self, value: f64) -> Self {
        self.max_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMethodsResponseLimitsStandard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`max_amount`](ListMethodsResponseLimitsStandardBuilder::max_amount)
    pub fn build(self) -> Result<ListMethodsResponseLimitsStandard, BuildError> {
        Ok(ListMethodsResponseLimitsStandard {
            error_code: self.error_code,
            error_message: self.error_message,
            max_amount: self
                .max_amount
                .ok_or_else(|| BuildError::missing_field("max_amount"))?,
        })
    }
}
