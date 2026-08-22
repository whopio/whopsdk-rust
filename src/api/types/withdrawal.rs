pub use crate::prelude::*;

/// A withdrawal represents a request to transfer funds from a ledger account to an external payout method.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Withdrawal {
    /// The withdrawal amount as a decimal number in the specified currency (e.g., 100.00 for $100.00 USD).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The datetime the withdrawal was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The three-letter ISO currency code for this withdrawal (e.g., 'usd', 'eur').
    pub currency: Currencies,
    /// A machine-readable error code describing why the payout failed. Null if no error occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<PayoutErrorCodes>,
    /// A human-readable message describing why the payout failed. Null if no error occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// The estimated time at which the funds become available in the destination account. Null if no estimate is available. As a Unix timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub estimated_availability: Option<DateTime<FixedOffset>>,
    /// The fee charged for processing this withdrawal, in the same currency as the withdrawal amount.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub fee_amount: f64,
    /// How the fee was applied to the withdrawal. 'exclusive' means the fee was added on top (user receives the full requested amount). 'inclusive' means the fee was deducted from the withdrawal (user receives less than requested). Null if no fee was charged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<WithdrawalFeeTypes>,
    /// The unique identifier for the withdrawal.
    #[serde(default)]
    pub id: String,
    /// The ledger account from which the withdrawal funds are sourced.
    #[serde(default)]
    pub ledger_account: WithdrawalLedgerAccount,
    /// An additional markup fee charged for the withdrawal, in the same currency as the withdrawal amount. Only applies to platform accounts using Whop Rails.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub markup_fee: f64,
    /// The id of the payout request (returned by POST /payouts) that this withdrawal settles. Null unless the withdrawal originated from a stablecoin payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_request_id: Option<String>,
    /// The saved payout destination used for this withdrawal (e.g., a bank account or PayPal address). Null if no payout token was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_token: Option<WithdrawalPayoutToken>,
    /// The processing speed selected for this withdrawal ('standard' or 'instant').
    pub speed: WithdrawalSpeeds,
    /// The computed lifecycle status of the withdrawal, accounting for the state of associated payouts (e.g., 'requested', 'in_transit', 'completed', 'failed').
    pub status: WithdrawalStatus,
    /// The ACH trace number for tracking the payout through the banking network. Null if not available or not an ACH transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_code: Option<String>,
}

impl Withdrawal {
    pub fn builder() -> WithdrawalBuilder {
        <WithdrawalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WithdrawalBuilder {
    amount: Option<f64>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    error_code: Option<PayoutErrorCodes>,
    error_message: Option<String>,
    estimated_availability: Option<DateTime<FixedOffset>>,
    fee_amount: Option<f64>,
    fee_type: Option<WithdrawalFeeTypes>,
    id: Option<String>,
    ledger_account: Option<WithdrawalLedgerAccount>,
    markup_fee: Option<f64>,
    payout_request_id: Option<String>,
    payout_token: Option<WithdrawalPayoutToken>,
    speed: Option<WithdrawalSpeeds>,
    status: Option<WithdrawalStatus>,
    trace_code: Option<String>,
}

impl WithdrawalBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn error_code(mut self, value: PayoutErrorCodes) -> Self {
        self.error_code = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn estimated_availability(mut self, value: DateTime<FixedOffset>) -> Self {
        self.estimated_availability = Some(value);
        self
    }

    pub fn fee_amount(mut self, value: f64) -> Self {
        self.fee_amount = Some(value);
        self
    }

    pub fn fee_type(mut self, value: WithdrawalFeeTypes) -> Self {
        self.fee_type = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn ledger_account(mut self, value: WithdrawalLedgerAccount) -> Self {
        self.ledger_account = Some(value);
        self
    }

    pub fn markup_fee(mut self, value: f64) -> Self {
        self.markup_fee = Some(value);
        self
    }

    pub fn payout_request_id(mut self, value: impl Into<String>) -> Self {
        self.payout_request_id = Some(value.into());
        self
    }

    pub fn payout_token(mut self, value: WithdrawalPayoutToken) -> Self {
        self.payout_token = Some(value);
        self
    }

    pub fn speed(mut self, value: WithdrawalSpeeds) -> Self {
        self.speed = Some(value);
        self
    }

    pub fn status(mut self, value: WithdrawalStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn trace_code(mut self, value: impl Into<String>) -> Self {
        self.trace_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Withdrawal`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](WithdrawalBuilder::amount)
    /// - [`created_at`](WithdrawalBuilder::created_at)
    /// - [`currency`](WithdrawalBuilder::currency)
    /// - [`fee_amount`](WithdrawalBuilder::fee_amount)
    /// - [`id`](WithdrawalBuilder::id)
    /// - [`ledger_account`](WithdrawalBuilder::ledger_account)
    /// - [`markup_fee`](WithdrawalBuilder::markup_fee)
    /// - [`speed`](WithdrawalBuilder::speed)
    /// - [`status`](WithdrawalBuilder::status)
    pub fn build(self) -> Result<Withdrawal, BuildError> {
        Ok(Withdrawal {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            error_code: self.error_code,
            error_message: self.error_message,
            estimated_availability: self.estimated_availability,
            fee_amount: self
                .fee_amount
                .ok_or_else(|| BuildError::missing_field("fee_amount"))?,
            fee_type: self.fee_type,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            ledger_account: self
                .ledger_account
                .ok_or_else(|| BuildError::missing_field("ledger_account"))?,
            markup_fee: self
                .markup_fee
                .ok_or_else(|| BuildError::missing_field("markup_fee"))?,
            payout_request_id: self.payout_request_id,
            payout_token: self.payout_token,
            speed: self
                .speed
                .ok_or_else(|| BuildError::missing_field("speed"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            trace_code: self.trace_code,
        })
    }
}
