pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisputeAlert {
    /// The account the alerted payment belongs to, prefixed `biz_`. `null` while the alert is unmatched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Whether refunding the payment can still avoid a chargeback. `false` once the payment has been disputed or fully refunded, or when the alert could not be matched to a payment — `not_actionable_reason` says which.
    #[serde(default)]
    pub actionable: bool,
    /// The alerted amount, in whole units of `currency`. This is what the issuer reported, which can differ from the payment's own amount.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The card network as reported by the issuer, lowercased, such as `visa` or `mastercard`. `unknown` when the report carries neither a network nor a recognizable BIN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    /// When Whop received the alert, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Three-letter ISO currency code of the alerted amount.
    #[serde(default)]
    pub currency: String,
    /// Whether Whop charged the account an alert fee for this one. Always `false` for `early_fraud_warning`, which Whop is not billed for and never passes on.
    #[serde(default)]
    pub fee_charged: bool,
    /// Dispute alert ID, prefixed `dspa_`.
    #[serde(default)]
    pub id: String,
    /// Name of the bank that issued the card and filed the report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// Why refunding can no longer avoid a chargeback. `network_resolved` when a Visa RDR already closed the case, `payment_unmatched` when no payment matched, `payment_not_captured` when it never captured money, `payment_disputed` once the payment carries a dispute, `payment_refunded` once fully refunded. `null` while `actionable` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_actionable_reason: Option<DisputeAlertNotActionableReason>,
    /// The payment the issuer reported, prefixed `pay_`. `null` when Whop could not match the report to a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// The product the alerted payment was for, prefixed `prod_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// When the issuer filed the report, as an ISO 8601 timestamp. Earlier than `created_at`, which is when Whop received it.
    #[serde(default)]
    pub reported_at: String,
    /// When the reported transaction was made, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_at: Option<String>,
    /// What the issuer sent. `early_fraud_warning` is a fraud report on a settled payment (Visa TC40 / Mastercard SAFE) — refunding still avoids the chargeback, and Whop never charges a fee for one. `dispute_alert` is a pre-dispute notice from the issuer's alert network, which Whop pays for and passes on as a fee. `rapid_dispute_resolution` is a Visa RDR case the network already closed by refunding the payment — nothing is left to act on.
    pub r#type: DisputeAlertType,
    /// When the alert was last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl DisputeAlert {
    pub fn builder() -> DisputeAlertBuilder {
        <DisputeAlertBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeAlertBuilder {
    account_id: Option<String>,
    actionable: Option<bool>,
    amount: Option<f64>,
    card_brand: Option<String>,
    created_at: Option<String>,
    currency: Option<String>,
    fee_charged: Option<bool>,
    id: Option<String>,
    issuer: Option<String>,
    not_actionable_reason: Option<DisputeAlertNotActionableReason>,
    payment_id: Option<String>,
    product_id: Option<String>,
    reported_at: Option<String>,
    transaction_at: Option<String>,
    r#type: Option<DisputeAlertType>,
    updated_at: Option<String>,
}

impl DisputeAlertBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn actionable(mut self, value: bool) -> Self {
        self.actionable = Some(value);
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn card_brand(mut self, value: impl Into<String>) -> Self {
        self.card_brand = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn fee_charged(mut self, value: bool) -> Self {
        self.fee_charged = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn issuer(mut self, value: impl Into<String>) -> Self {
        self.issuer = Some(value.into());
        self
    }

    pub fn not_actionable_reason(mut self, value: DisputeAlertNotActionableReason) -> Self {
        self.not_actionable_reason = Some(value);
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn reported_at(mut self, value: impl Into<String>) -> Self {
        self.reported_at = Some(value.into());
        self
    }

    pub fn transaction_at(mut self, value: impl Into<String>) -> Self {
        self.transaction_at = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: DisputeAlertType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeAlert`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actionable`](DisputeAlertBuilder::actionable)
    /// - [`amount`](DisputeAlertBuilder::amount)
    /// - [`created_at`](DisputeAlertBuilder::created_at)
    /// - [`currency`](DisputeAlertBuilder::currency)
    /// - [`fee_charged`](DisputeAlertBuilder::fee_charged)
    /// - [`id`](DisputeAlertBuilder::id)
    /// - [`reported_at`](DisputeAlertBuilder::reported_at)
    /// - [`r#type`](DisputeAlertBuilder::r#type)
    /// - [`updated_at`](DisputeAlertBuilder::updated_at)
    pub fn build(self) -> Result<DisputeAlert, BuildError> {
        Ok(DisputeAlert {
            account_id: self.account_id,
            actionable: self
                .actionable
                .ok_or_else(|| BuildError::missing_field("actionable"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            card_brand: self.card_brand,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            fee_charged: self
                .fee_charged
                .ok_or_else(|| BuildError::missing_field("fee_charged"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            issuer: self.issuer,
            not_actionable_reason: self.not_actionable_reason,
            payment_id: self.payment_id,
            product_id: self.product_id,
            reported_at: self
                .reported_at
                .ok_or_else(|| BuildError::missing_field("reported_at"))?,
            transaction_at: self.transaction_at,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
