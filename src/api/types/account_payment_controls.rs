pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountPaymentControls {
    /// Automatic refund settings for pre-chargeback dispute alerts.
    #[serde(default)]
    pub dispute_alert_auto_refund: AccountDisputeAlertAutoRefundControl,
    /// Fee charged for each dispute alert in USD. `null` when unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dispute_alert_fee_usd: Option<f64>,
    /// Whether 3-D Secure is forced on every card payment at checkout. The account cannot bypass it while set.
    #[serde(rename = "enforce_3ds")]
    #[serde(default)]
    pub enforce3ds: bool,
    /// Whether payment health controls explicitly disable financing. This is independent of financing approval in `capabilities.accept_bnpl_payments`.
    #[serde(default)]
    pub financing_disabled: bool,
    /// Additional processing fee percentage for high-risk processing.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub high_risk_processing_fee_percentage: f64,
    /// Percentage fee charged when pending, not-yet-settled balance is advanced to fund the account's cards balance, where `2` means 2%. `0` when the account is exempt.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub pending_auto_topup_fee_percentage: f64,
    /// Additional days payments remain pending before becoming available.
    #[serde(default)]
    pub pending_balance_delay_days: i64,
    /// Reserve currently applied to incoming payment volume.
    #[serde(default)]
    pub reserve: AccountReserveControl,
    /// Automatic refund settings for resolution center cases.
    #[serde(default)]
    pub resolution_center_auto_refund: AccountResolutionCenterAutoRefundControl,
    #[serde(default)]
    pub restricted_payment_methods: Vec<AccountPaymentControlsRestrictedPaymentMethodsItem>,
    /// Why pending funds without a settlement date aren't moving yet, when it's something the merchant can act on. `null` when there's no reason to show (still clearing, or the account is held for a reason that isn't merchant-actionable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undated_pending_reason: Option<AccountPaymentControlsUndatedPendingReason>,
    /// How the account's balance automatically withdraws.
    pub withdrawal_schedule: AccountWithdrawalScheduleControl,
}

impl AccountPaymentControls {
    pub fn builder() -> AccountPaymentControlsBuilder {
        <AccountPaymentControlsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountPaymentControlsBuilder {
    dispute_alert_auto_refund: Option<AccountDisputeAlertAutoRefundControl>,
    dispute_alert_fee_usd: Option<f64>,
    enforce3ds: Option<bool>,
    financing_disabled: Option<bool>,
    high_risk_processing_fee_percentage: Option<f64>,
    pending_auto_topup_fee_percentage: Option<f64>,
    pending_balance_delay_days: Option<i64>,
    reserve: Option<AccountReserveControl>,
    resolution_center_auto_refund: Option<AccountResolutionCenterAutoRefundControl>,
    restricted_payment_methods: Option<Vec<AccountPaymentControlsRestrictedPaymentMethodsItem>>,
    undated_pending_reason: Option<AccountPaymentControlsUndatedPendingReason>,
    withdrawal_schedule: Option<AccountWithdrawalScheduleControl>,
}

impl AccountPaymentControlsBuilder {
    pub fn dispute_alert_auto_refund(
        mut self,
        value: AccountDisputeAlertAutoRefundControl,
    ) -> Self {
        self.dispute_alert_auto_refund = Some(value);
        self
    }

    pub fn dispute_alert_fee_usd(mut self, value: f64) -> Self {
        self.dispute_alert_fee_usd = Some(value);
        self
    }

    pub fn enforce3ds(mut self, value: bool) -> Self {
        self.enforce3ds = Some(value);
        self
    }

    pub fn financing_disabled(mut self, value: bool) -> Self {
        self.financing_disabled = Some(value);
        self
    }

    pub fn high_risk_processing_fee_percentage(mut self, value: f64) -> Self {
        self.high_risk_processing_fee_percentage = Some(value);
        self
    }

    pub fn pending_auto_topup_fee_percentage(mut self, value: f64) -> Self {
        self.pending_auto_topup_fee_percentage = Some(value);
        self
    }

    pub fn pending_balance_delay_days(mut self, value: i64) -> Self {
        self.pending_balance_delay_days = Some(value);
        self
    }

    pub fn reserve(mut self, value: AccountReserveControl) -> Self {
        self.reserve = Some(value);
        self
    }

    pub fn resolution_center_auto_refund(
        mut self,
        value: AccountResolutionCenterAutoRefundControl,
    ) -> Self {
        self.resolution_center_auto_refund = Some(value);
        self
    }

    pub fn restricted_payment_methods(
        mut self,
        value: Vec<AccountPaymentControlsRestrictedPaymentMethodsItem>,
    ) -> Self {
        self.restricted_payment_methods = Some(value);
        self
    }

    pub fn undated_pending_reason(
        mut self,
        value: AccountPaymentControlsUndatedPendingReason,
    ) -> Self {
        self.undated_pending_reason = Some(value);
        self
    }

    pub fn withdrawal_schedule(mut self, value: AccountWithdrawalScheduleControl) -> Self {
        self.withdrawal_schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountPaymentControls`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dispute_alert_auto_refund`](AccountPaymentControlsBuilder::dispute_alert_auto_refund)
    /// - [`enforce3ds`](AccountPaymentControlsBuilder::enforce3ds)
    /// - [`financing_disabled`](AccountPaymentControlsBuilder::financing_disabled)
    /// - [`high_risk_processing_fee_percentage`](AccountPaymentControlsBuilder::high_risk_processing_fee_percentage)
    /// - [`pending_auto_topup_fee_percentage`](AccountPaymentControlsBuilder::pending_auto_topup_fee_percentage)
    /// - [`pending_balance_delay_days`](AccountPaymentControlsBuilder::pending_balance_delay_days)
    /// - [`reserve`](AccountPaymentControlsBuilder::reserve)
    /// - [`resolution_center_auto_refund`](AccountPaymentControlsBuilder::resolution_center_auto_refund)
    /// - [`restricted_payment_methods`](AccountPaymentControlsBuilder::restricted_payment_methods)
    /// - [`withdrawal_schedule`](AccountPaymentControlsBuilder::withdrawal_schedule)
    pub fn build(self) -> Result<AccountPaymentControls, BuildError> {
        Ok(AccountPaymentControls {
            dispute_alert_auto_refund: self
                .dispute_alert_auto_refund
                .ok_or_else(|| BuildError::missing_field("dispute_alert_auto_refund"))?,
            dispute_alert_fee_usd: self.dispute_alert_fee_usd,
            enforce3ds: self
                .enforce3ds
                .ok_or_else(|| BuildError::missing_field("enforce3ds"))?,
            financing_disabled: self
                .financing_disabled
                .ok_or_else(|| BuildError::missing_field("financing_disabled"))?,
            high_risk_processing_fee_percentage: self
                .high_risk_processing_fee_percentage
                .ok_or_else(|| BuildError::missing_field("high_risk_processing_fee_percentage"))?,
            pending_auto_topup_fee_percentage: self
                .pending_auto_topup_fee_percentage
                .ok_or_else(|| BuildError::missing_field("pending_auto_topup_fee_percentage"))?,
            pending_balance_delay_days: self
                .pending_balance_delay_days
                .ok_or_else(|| BuildError::missing_field("pending_balance_delay_days"))?,
            reserve: self
                .reserve
                .ok_or_else(|| BuildError::missing_field("reserve"))?,
            resolution_center_auto_refund: self
                .resolution_center_auto_refund
                .ok_or_else(|| BuildError::missing_field("resolution_center_auto_refund"))?,
            restricted_payment_methods: self
                .restricted_payment_methods
                .ok_or_else(|| BuildError::missing_field("restricted_payment_methods"))?,
            undated_pending_reason: self.undated_pending_reason,
            withdrawal_schedule: self
                .withdrawal_schedule
                .ok_or_else(|| BuildError::missing_field("withdrawal_schedule"))?,
        })
    }
}
