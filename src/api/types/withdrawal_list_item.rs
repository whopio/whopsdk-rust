pub use crate::prelude::*;

/// A withdrawal represents a request to transfer funds from a ledger account to an external payout method.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WithdrawalListItem {
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
    /// An additional markup fee charged for the withdrawal, in the same currency as the withdrawal amount. Only applies to platform accounts using Whop Rails.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub markup_fee: f64,
    /// The id of the payout request (returned by POST /payouts) that this withdrawal settles. Null unless the withdrawal originated from a stablecoin payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_request_id: Option<String>,
    /// The processing speed selected for this withdrawal ('standard' or 'instant').
    pub speed: WithdrawalSpeeds,
    /// The computed lifecycle status of the withdrawal, accounting for the state of associated payouts (e.g., 'requested', 'in_transit', 'completed', 'failed').
    pub status: WithdrawalStatus,
}

impl WithdrawalListItem {
    pub fn builder() -> WithdrawalListItemBuilder {
        <WithdrawalListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WithdrawalListItemBuilder {
    amount: Option<f64>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    fee_amount: Option<f64>,
    fee_type: Option<WithdrawalFeeTypes>,
    id: Option<String>,
    markup_fee: Option<f64>,
    payout_request_id: Option<String>,
    speed: Option<WithdrawalSpeeds>,
    status: Option<WithdrawalStatus>,
}

impl WithdrawalListItemBuilder {
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

    pub fn markup_fee(mut self, value: f64) -> Self {
        self.markup_fee = Some(value);
        self
    }

    pub fn payout_request_id(mut self, value: impl Into<String>) -> Self {
        self.payout_request_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`WithdrawalListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](WithdrawalListItemBuilder::amount)
    /// - [`created_at`](WithdrawalListItemBuilder::created_at)
    /// - [`currency`](WithdrawalListItemBuilder::currency)
    /// - [`fee_amount`](WithdrawalListItemBuilder::fee_amount)
    /// - [`id`](WithdrawalListItemBuilder::id)
    /// - [`markup_fee`](WithdrawalListItemBuilder::markup_fee)
    /// - [`speed`](WithdrawalListItemBuilder::speed)
    /// - [`status`](WithdrawalListItemBuilder::status)
    pub fn build(self) -> Result<WithdrawalListItem, BuildError> {
        Ok(WithdrawalListItem {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            fee_amount: self
                .fee_amount
                .ok_or_else(|| BuildError::missing_field("fee_amount"))?,
            fee_type: self.fee_type,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            markup_fee: self
                .markup_fee
                .ok_or_else(|| BuildError::missing_field("markup_fee"))?,
            payout_request_id: self.payout_request_id,
            speed: self
                .speed
                .ok_or_else(|| BuildError::missing_field("speed"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
