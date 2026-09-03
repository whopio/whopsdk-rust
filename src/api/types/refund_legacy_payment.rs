pub use crate::prelude::*;

/// The original payment that this refund was issued against. Null if the payment is no longer available.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefundLegacyPayment {
    /// The machine-readable reason this charge was created, such as initial subscription purchase, renewal cycle, or one-time payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reason: Option<BillingReasons>,
    /// Card network reported by the processor (e.g., 'visa', 'mastercard', 'amex'). Present only when the payment method type is 'card'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<CardBrands>,
    /// The last four digits of the card used to make this payment. Null if the payment was not made with a card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_last4: Option<String>,
    /// The datetime the payment was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The three-letter ISO currency code for this payment (e.g., 'usd', 'eur').
    pub currency: Currencies,
    /// When an alert came in that this transaction will be disputed
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub dispute_alerted_at: Option<DateTime<FixedOffset>>,
    /// The unique identifier for the payment.
    #[serde(default)]
    pub id: String,
    /// The member attached to this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<RefundLegacyPaymentMember>,
    /// The membership attached to this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership: Option<RefundLegacyPaymentMembership>,
    /// The custom metadata stored on this payment. This will be copied over to the checkout configuration for which this payment was made
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The time at which this payment was successfully collected. Null if the payment has not yet succeeded. As a Unix timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub paid_at: Option<DateTime<FixedOffset>>,
    /// The type of payment instrument used for this payment (e.g., card, Cash App, iDEAL, Klarna, crypto). Null when the processor does not supply a type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<PaymentMethodTypes>,
    /// The plan attached to this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<RefundLegacyPaymentPlan>,
    /// The product this payment was made for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<RefundLegacyPaymentProduct>,
    /// The subtotal to show to the creator (excluding buyer fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub subtotal: Option<f64>,
    /// The calculated amount of the sales/VAT tax (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub tax_amount: Option<f64>,
    /// The type of tax inclusivity applied to the payment, for determining whether the tax is included in the final price, or paid on top.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<ReceiptTaxBehaviors>,
    /// The amount of tax that has been refunded (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub tax_refunded_amount: Option<f64>,
    /// The total to show to the creator (excluding buyer fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total: Option<f64>,
    /// The total in USD to show to the creator (excluding buyer fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_total: Option<f64>,
    /// The user that made this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<RefundLegacyPaymentUser>,
}

impl RefundLegacyPayment {
    pub fn builder() -> RefundLegacyPaymentBuilder {
        <RefundLegacyPaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundLegacyPaymentBuilder {
    billing_reason: Option<BillingReasons>,
    card_brand: Option<CardBrands>,
    card_last4: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    dispute_alerted_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    member: Option<RefundLegacyPaymentMember>,
    membership: Option<RefundLegacyPaymentMembership>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    paid_at: Option<DateTime<FixedOffset>>,
    payment_method_type: Option<PaymentMethodTypes>,
    plan: Option<RefundLegacyPaymentPlan>,
    product: Option<RefundLegacyPaymentProduct>,
    subtotal: Option<f64>,
    tax_amount: Option<f64>,
    tax_behavior: Option<ReceiptTaxBehaviors>,
    tax_refunded_amount: Option<f64>,
    total: Option<f64>,
    usd_total: Option<f64>,
    user: Option<RefundLegacyPaymentUser>,
}

impl RefundLegacyPaymentBuilder {
    pub fn billing_reason(mut self, value: BillingReasons) -> Self {
        self.billing_reason = Some(value);
        self
    }

    pub fn card_brand(mut self, value: CardBrands) -> Self {
        self.card_brand = Some(value);
        self
    }

    pub fn card_last4(mut self, value: impl Into<String>) -> Self {
        self.card_last4 = Some(value.into());
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

    pub fn dispute_alerted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.dispute_alerted_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn member(mut self, value: RefundLegacyPaymentMember) -> Self {
        self.member = Some(value);
        self
    }

    pub fn membership(mut self, value: RefundLegacyPaymentMembership) -> Self {
        self.membership = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn paid_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.paid_at = Some(value);
        self
    }

    pub fn payment_method_type(mut self, value: PaymentMethodTypes) -> Self {
        self.payment_method_type = Some(value);
        self
    }

    pub fn plan(mut self, value: RefundLegacyPaymentPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn product(mut self, value: RefundLegacyPaymentProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn subtotal(mut self, value: f64) -> Self {
        self.subtotal = Some(value);
        self
    }

    pub fn tax_amount(mut self, value: f64) -> Self {
        self.tax_amount = Some(value);
        self
    }

    pub fn tax_behavior(mut self, value: ReceiptTaxBehaviors) -> Self {
        self.tax_behavior = Some(value);
        self
    }

    pub fn tax_refunded_amount(mut self, value: f64) -> Self {
        self.tax_refunded_amount = Some(value);
        self
    }

    pub fn total(mut self, value: f64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn usd_total(mut self, value: f64) -> Self {
        self.usd_total = Some(value);
        self
    }

    pub fn user(mut self, value: RefundLegacyPaymentUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundLegacyPayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](RefundLegacyPaymentBuilder::created_at)
    /// - [`currency`](RefundLegacyPaymentBuilder::currency)
    /// - [`id`](RefundLegacyPaymentBuilder::id)
    pub fn build(self) -> Result<RefundLegacyPayment, BuildError> {
        Ok(RefundLegacyPayment {
            billing_reason: self.billing_reason,
            card_brand: self.card_brand,
            card_last4: self.card_last4,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            dispute_alerted_at: self.dispute_alerted_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            member: self.member,
            membership: self.membership,
            metadata: self.metadata,
            paid_at: self.paid_at,
            payment_method_type: self.payment_method_type,
            plan: self.plan,
            product: self.product,
            subtotal: self.subtotal,
            tax_amount: self.tax_amount,
            tax_behavior: self.tax_behavior,
            tax_refunded_amount: self.tax_refunded_amount,
            total: self.total,
            usd_total: self.usd_total,
            user: self.user,
        })
    }
}
