pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Refund {
    /// The account that issued the refund, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The refunded amount as it settled, in the payment's settlement currency, so pages of refunds net against the payment's `refunded_amount`. Converted at the rate in force when the refund was issued, not the payment's original rate. Null only when no exchange rate is recorded for a legacy multi-currency payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// When the refund was requested, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// The provider's own explanation of the failure, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// Why the refund failed, normalized across providers. Null unless the refund failed or was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<RefundFailureReason>,
    /// Refund ID, prefixed `rf_`.
    #[serde(default)]
    pub id: String,
    /// The refunded amount in the currency the processor moved.
    #[serde(default)]
    pub original_amount: Money,
    /// The payment this refund reverses, prefixed `pay_`.
    #[serde(default)]
    pub payment_id: String,
    /// The payment provider that processed the refund, such as `paypal` or `coinbase`.
    #[serde(default)]
    pub provider: String,
    /// When the provider created the refund, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_created_at: Option<String>,
    /// Why the refund was issued, when recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<RefundReason>,
    /// Whether a banking-network tracking reference is available for this refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_status: Option<RefundReferenceStatus>,
    /// The kind of tracking reference, such as an acquirer reference number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<RefundReferenceType>,
    /// The tracking reference the buyer's bank can trace the refund by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value: Option<String>,
    /// Where the refund stands with the processor: `pending`, `requires_action`, `succeeded`, `failed`, or `canceled`.
    pub status: RefundStatus,
    /// When the refund last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// True when the card network initiated the refund through Rapid Dispute Resolution.
    #[serde(default)]
    pub visa_rdr: bool,
}

impl Refund {
    pub fn builder() -> RefundBuilder {
        <RefundBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundBuilder {
    account_id: Option<String>,
    amount: Option<Money>,
    created_at: Option<String>,
    failure_message: Option<String>,
    failure_reason: Option<RefundFailureReason>,
    id: Option<String>,
    original_amount: Option<Money>,
    payment_id: Option<String>,
    provider: Option<String>,
    provider_created_at: Option<String>,
    reason: Option<RefundReason>,
    reference_status: Option<RefundReferenceStatus>,
    reference_type: Option<RefundReferenceType>,
    reference_value: Option<String>,
    status: Option<RefundStatus>,
    updated_at: Option<String>,
    visa_rdr: Option<bool>,
}

impl RefundBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn failure_message(mut self, value: impl Into<String>) -> Self {
        self.failure_message = Some(value.into());
        self
    }

    pub fn failure_reason(mut self, value: RefundFailureReason) -> Self {
        self.failure_reason = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn original_amount(mut self, value: Money) -> Self {
        self.original_amount = Some(value);
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn provider_created_at(mut self, value: impl Into<String>) -> Self {
        self.provider_created_at = Some(value.into());
        self
    }

    pub fn reason(mut self, value: RefundReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn reference_status(mut self, value: RefundReferenceStatus) -> Self {
        self.reference_status = Some(value);
        self
    }

    pub fn reference_type(mut self, value: RefundReferenceType) -> Self {
        self.reference_type = Some(value);
        self
    }

    pub fn reference_value(mut self, value: impl Into<String>) -> Self {
        self.reference_value = Some(value.into());
        self
    }

    pub fn status(mut self, value: RefundStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn visa_rdr(mut self, value: bool) -> Self {
        self.visa_rdr = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Refund`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](RefundBuilder::created_at)
    /// - [`id`](RefundBuilder::id)
    /// - [`original_amount`](RefundBuilder::original_amount)
    /// - [`payment_id`](RefundBuilder::payment_id)
    /// - [`provider`](RefundBuilder::provider)
    /// - [`status`](RefundBuilder::status)
    /// - [`updated_at`](RefundBuilder::updated_at)
    /// - [`visa_rdr`](RefundBuilder::visa_rdr)
    pub fn build(self) -> Result<Refund, BuildError> {
        Ok(Refund {
            account_id: self.account_id,
            amount: self.amount,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            failure_message: self.failure_message,
            failure_reason: self.failure_reason,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            original_amount: self
                .original_amount
                .ok_or_else(|| BuildError::missing_field("original_amount"))?,
            payment_id: self
                .payment_id
                .ok_or_else(|| BuildError::missing_field("payment_id"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            provider_created_at: self.provider_created_at,
            reason: self.reason,
            reference_status: self.reference_status,
            reference_type: self.reference_type,
            reference_value: self.reference_value,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            visa_rdr: self
                .visa_rdr
                .ok_or_else(|| BuildError::missing_field("visa_rdr"))?,
        })
    }
}
