pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LedgerActivityPayment {
    /// Total charged by the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// Card brand, when the customer paid by card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    /// Last four digits of the card, when the customer paid by card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_last4: Option<String>,
    /// When the payment was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Payment ID, prefixed `pay_`.
    #[serde(default)]
    pub id: String,
    pub object: LedgerActivityPaymentObject,
    /// How the customer paid, such as `card` or `paypal`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    /// Processor that handled the payment, such as `stripe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_processor: Option<String>,
    /// Plan associated with the payment, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<LedgerActivityPaymentPlan>,
    /// Product associated with the payment, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<LedgerActivityPaymentProduct>,
    /// Customer associated with the payment. Email requires member:email:read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<LedgerActivityPaymentUser>,
}

impl LedgerActivityPayment {
    pub fn builder() -> LedgerActivityPaymentBuilder {
        <LedgerActivityPaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityPaymentBuilder {
    amount: Option<Money>,
    card_brand: Option<String>,
    card_last4: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    object: Option<LedgerActivityPaymentObject>,
    payment_method_type: Option<String>,
    payment_processor: Option<String>,
    plan: Option<LedgerActivityPaymentPlan>,
    product: Option<LedgerActivityPaymentProduct>,
    user: Option<LedgerActivityPaymentUser>,
}

impl LedgerActivityPaymentBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn card_brand(mut self, value: impl Into<String>) -> Self {
        self.card_brand = Some(value.into());
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: LedgerActivityPaymentObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn payment_method_type(mut self, value: impl Into<String>) -> Self {
        self.payment_method_type = Some(value.into());
        self
    }

    pub fn payment_processor(mut self, value: impl Into<String>) -> Self {
        self.payment_processor = Some(value.into());
        self
    }

    pub fn plan(mut self, value: LedgerActivityPaymentPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn product(mut self, value: LedgerActivityPaymentProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn user(mut self, value: LedgerActivityPaymentUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LedgerActivityPayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](LedgerActivityPaymentBuilder::created_at)
    /// - [`id`](LedgerActivityPaymentBuilder::id)
    /// - [`object`](LedgerActivityPaymentBuilder::object)
    pub fn build(self) -> Result<LedgerActivityPayment, BuildError> {
        Ok(LedgerActivityPayment {
            amount: self.amount,
            card_brand: self.card_brand,
            card_last4: self.card_last4,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            payment_method_type: self.payment_method_type,
            payment_processor: self.payment_processor,
            plan: self.plan,
            product: self.product,
            user: self.user,
        })
    }
}
