pub use crate::prelude::*;

/// Caps for instant-speed payouts, which additionally draw on pending funds.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMethodsResponseLimitsInstant {
    /// How much of the account's daily instant allowance is left, in the requested currency. Null when the account is exempt from the daily cap, which is not the same as 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub daily_amount_remaining: Option<f64>,
    /// Why instant payouts are unavailable for this account, or null when they are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<ListMethodsResponseLimitsInstantErrorCode>,
    /// Human-readable form of error_code, or null when instant payouts are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// The maximum amount an instant payout can move right now, in whole currency units. Already bounded by the remaining daily instant allowance; 0 while an eligibility error blocks instant payouts.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub max_amount: f64,
    /// When the daily instant allowance resets, or null when the account is exempt from the daily cap.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub resets_at: Option<DateTime<FixedOffset>>,
}

impl ListMethodsResponseLimitsInstant {
    pub fn builder() -> ListMethodsResponseLimitsInstantBuilder {
        <ListMethodsResponseLimitsInstantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMethodsResponseLimitsInstantBuilder {
    daily_amount_remaining: Option<f64>,
    error_code: Option<ListMethodsResponseLimitsInstantErrorCode>,
    error_message: Option<String>,
    max_amount: Option<f64>,
    resets_at: Option<DateTime<FixedOffset>>,
}

impl ListMethodsResponseLimitsInstantBuilder {
    pub fn daily_amount_remaining(mut self, value: f64) -> Self {
        self.daily_amount_remaining = Some(value);
        self
    }

    pub fn error_code(mut self, value: ListMethodsResponseLimitsInstantErrorCode) -> Self {
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

    pub fn resets_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.resets_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMethodsResponseLimitsInstant`].
    /// This method will fail if any of the following fields are not set:
    /// - [`max_amount`](ListMethodsResponseLimitsInstantBuilder::max_amount)
    pub fn build(self) -> Result<ListMethodsResponseLimitsInstant, BuildError> {
        Ok(ListMethodsResponseLimitsInstant {
            daily_amount_remaining: self.daily_amount_remaining,
            error_code: self.error_code,
            error_message: self.error_message,
            max_amount: self
                .max_amount
                .ok_or_else(|| BuildError::missing_field("max_amount"))?,
            resets_at: self.resets_at,
        })
    }
}
