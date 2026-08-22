pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountResolutionCenterAutoRefundControl {
    /// Maximum card-funded resolution center case amount automatically refunded in USD. `null` when automatic refunds are disabled for cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub card_threshold_usd: Option<f64>,
    /// Maximum financing-funded resolution center case amount automatically refunded in USD. `null` when automatic refunds are disabled for financing.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub financing_threshold_usd: Option<f64>,
    /// Whether the account owner is prevented from changing these thresholds.
    #[serde(default)]
    pub locked: bool,
    /// Maximum PayPal-funded resolution center case amount automatically refunded in USD. `null` when automatic refunds are disabled for PayPal.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub paypal_threshold_usd: Option<f64>,
}

impl AccountResolutionCenterAutoRefundControl {
    pub fn builder() -> AccountResolutionCenterAutoRefundControlBuilder {
        <AccountResolutionCenterAutoRefundControlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountResolutionCenterAutoRefundControlBuilder {
    card_threshold_usd: Option<f64>,
    financing_threshold_usd: Option<f64>,
    locked: Option<bool>,
    paypal_threshold_usd: Option<f64>,
}

impl AccountResolutionCenterAutoRefundControlBuilder {
    pub fn card_threshold_usd(mut self, value: f64) -> Self {
        self.card_threshold_usd = Some(value);
        self
    }

    pub fn financing_threshold_usd(mut self, value: f64) -> Self {
        self.financing_threshold_usd = Some(value);
        self
    }

    pub fn locked(mut self, value: bool) -> Self {
        self.locked = Some(value);
        self
    }

    pub fn paypal_threshold_usd(mut self, value: f64) -> Self {
        self.paypal_threshold_usd = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountResolutionCenterAutoRefundControl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`locked`](AccountResolutionCenterAutoRefundControlBuilder::locked)
    pub fn build(self) -> Result<AccountResolutionCenterAutoRefundControl, BuildError> {
        Ok(AccountResolutionCenterAutoRefundControl {
            card_threshold_usd: self.card_threshold_usd,
            financing_threshold_usd: self.financing_threshold_usd,
            locked: self
                .locked
                .ok_or_else(|| BuildError::missing_field("locked"))?,
            paypal_threshold_usd: self.paypal_threshold_usd,
        })
    }
}
