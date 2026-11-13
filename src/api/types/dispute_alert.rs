pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisputeAlert {
    /// The account the alerted payment belongs to, prefixed `biz_`. `null` while the alert is unmatched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The alerted amount, in whole units of `currency`. This is what the issuer reported, which can differ from the payment's own amount.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// Whether Whop automatically refunded the alerted payment. Reflects the payment, so it can be `true` for a refund issued by another flow (RDR, resolution) on the same payment.
    #[serde(default)]
    pub auto_refunded: bool,
    /// The card network as reported by the issuer, lowercased, such as `visa` or `mastercard`. `unknown` when the report carries neither a network nor a recognizable BIN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    /// What the cardholder claims. `fraud` when they say they didn't make the payment: always for an `early_fraud_warning`, and for Ethoca fraud alerts and Visa RDR cases filed under a fraud reason. `customer_dispute` when they made the payment but dispute it, such as a product not received or a canceled subscription. `null` when the alert carries no claim, such as an issuer's request for transaction documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<DisputeAlertCategory>,
    /// When Whop received the alert, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Three-letter ISO currency code of the alerted amount.
    #[serde(default)]
    pub currency: String,
    /// The alert fee charged to the account, in the currency it was collected in. `null` when `fee_charged` is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<Money>,
    /// Whether Whop charged the account an alert fee for this one. Always `false` for `early_fraud_warning`, which Whop is not billed for and never passes on.
    #[serde(default)]
    pub fee_charged: bool,
    /// Dispute alert ID, prefixed `dspa_`.
    #[serde(default)]
    pub id: String,
    /// Deprecated: always `null` outside Whop's own dashboard. Name of the bank that issued the card and filed the report.
    /// DEPRECATED: Always null outside Whop's own dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// The payment the issuer reported, prefixed `pay_`. `null` when Whop could not match the report to a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// The product the alerted payment was for, prefixed `prod_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// When the issuer filed the report, as an ISO 8601 timestamp. Earlier than `created_at`, which is when Whop received it.
    #[serde(default)]
    pub reported_at: String,
    /// The network program that sent the alert, which `type` alone doesn't tell apart. `ethoca` for Mastercard Ethoca, `cdrn` for Visa Verifi CDRN, `rdr` for Visa RDR, `tc40` / `safe` for a Visa / Mastercard fraud report, `paypal` for a PayPal alert. `null` when the network didn't say.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<DisputeAlertSource>,
    /// When the reported transaction was made, as an ISO 8601 timestamp — falls back to when the matched payment was made if the issuer's own report didn't carry one. Should not be `null` in practice; treat one as a data issue rather than expected behavior.
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
    amount: Option<f64>,
    auto_refunded: Option<bool>,
    card_brand: Option<String>,
    category: Option<DisputeAlertCategory>,
    created_at: Option<String>,
    currency: Option<String>,
    fee: Option<Money>,
    fee_charged: Option<bool>,
    id: Option<String>,
    issuer: Option<String>,
    payment_id: Option<String>,
    product_id: Option<String>,
    reported_at: Option<String>,
    source: Option<DisputeAlertSource>,
    transaction_at: Option<String>,
    r#type: Option<DisputeAlertType>,
    updated_at: Option<String>,
}

impl DisputeAlertBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn auto_refunded(mut self, value: bool) -> Self {
        self.auto_refunded = Some(value);
        self
    }

    pub fn card_brand(mut self, value: impl Into<String>) -> Self {
        self.card_brand = Some(value.into());
        self
    }

    pub fn category(mut self, value: DisputeAlertCategory) -> Self {
        self.category = Some(value);
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

    pub fn fee(mut self, value: Money) -> Self {
        self.fee = Some(value);
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

    pub fn source(mut self, value: DisputeAlertSource) -> Self {
        self.source = Some(value);
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
    /// - [`amount`](DisputeAlertBuilder::amount)
    /// - [`auto_refunded`](DisputeAlertBuilder::auto_refunded)
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
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            auto_refunded: self
                .auto_refunded
                .ok_or_else(|| BuildError::missing_field("auto_refunded"))?,
            card_brand: self.card_brand,
            category: self.category,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            fee: self.fee,
            fee_charged: self
                .fee_charged
                .ok_or_else(|| BuildError::missing_field("fee_charged"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            issuer: self.issuer,
            payment_id: self.payment_id,
            product_id: self.product_id,
            reported_at: self
                .reported_at
                .ok_or_else(|| BuildError::missing_field("reported_at"))?,
            source: self.source,
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
