pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DisputePayment {
    /// What the customer was charged, in whole units of the payment's currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// Card brand, when the customer paid by card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    /// Last four digits of the card, when the customer paid by card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_last4: Option<String>,
    /// When the payment was made, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Three-letter ISO currency code of the payment. Can differ from the dispute's currency when the processor settles in another currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Payment ID, prefixed `pay_`.
    #[serde(default)]
    pub id: String,
    /// The instrument this payment was made with, shaped for display: the method type, a buyer-facing name, the standard icon set, and the card facts when it was a card. Null when the payment names no method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_instrument: Option<PaymentInstrument>,
    /// How the customer paid, such as `card` or `paypal`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    /// The processor that handled the payment, such as `stripe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_processor: Option<String>,
}

impl DisputePayment {
    pub fn builder() -> DisputePaymentBuilder {
        <DisputePaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputePaymentBuilder {
    amount: Option<f64>,
    card_brand: Option<String>,
    card_last4: Option<String>,
    created_at: Option<String>,
    currency: Option<String>,
    id: Option<String>,
    payment_instrument: Option<PaymentInstrument>,
    payment_method_type: Option<String>,
    payment_processor: Option<String>,
}

impl DisputePaymentBuilder {
    pub fn amount(mut self, value: f64) -> Self {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn payment_instrument(mut self, value: PaymentInstrument) -> Self {
        self.payment_instrument = Some(value);
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

    /// Consumes the builder and constructs a [`DisputePayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](DisputePaymentBuilder::created_at)
    /// - [`id`](DisputePaymentBuilder::id)
    pub fn build(self) -> Result<DisputePayment, BuildError> {
        Ok(DisputePayment {
            amount: self.amount,
            card_brand: self.card_brand,
            card_last4: self.card_last4,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self.currency,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            payment_instrument: self.payment_instrument,
            payment_method_type: self.payment_method_type,
            payment_processor: self.payment_processor,
        })
    }
}
