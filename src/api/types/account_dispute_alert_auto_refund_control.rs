pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountDisputeAlertAutoRefundControl {
    /// Whether the account owner is prevented from changing this threshold.
    #[serde(default)]
    pub locked: bool,
    /// Maximum dispute alert amount automatically refunded in USD. `null` when automatic refunds are disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub threshold_usd: Option<f64>,
}

impl AccountDisputeAlertAutoRefundControl {
    pub fn builder() -> AccountDisputeAlertAutoRefundControlBuilder {
        <AccountDisputeAlertAutoRefundControlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountDisputeAlertAutoRefundControlBuilder {
    locked: Option<bool>,
    threshold_usd: Option<f64>,
}

impl AccountDisputeAlertAutoRefundControlBuilder {
    pub fn locked(mut self, value: bool) -> Self {
        self.locked = Some(value);
        self
    }

    pub fn threshold_usd(mut self, value: f64) -> Self {
        self.threshold_usd = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountDisputeAlertAutoRefundControl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`locked`](AccountDisputeAlertAutoRefundControlBuilder::locked)
    pub fn build(self) -> Result<AccountDisputeAlertAutoRefundControl, BuildError> {
        Ok(AccountDisputeAlertAutoRefundControl {
            locked: self
                .locked
                .ok_or_else(|| BuildError::missing_field("locked"))?,
            threshold_usd: self.threshold_usd,
        })
    }
}
