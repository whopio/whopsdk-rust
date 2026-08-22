pub use crate::prelude::*;

/// The tokenized payment method reference used for this payment. Null if no token was used.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentPaymentMethod {
    /// The card data associated with the payment method, if its a debit or credit card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentPaymentMethodCard>,
    /// The datetime the payment token was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The unique identifier for the payment token.
    #[serde(default)]
    pub id: String,
    /// The payment method type of the payment method
    pub payment_method_type: PaymentMethodTypes,
}

impl PaymentPaymentMethod {
    pub fn builder() -> PaymentPaymentMethodBuilder {
        <PaymentPaymentMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentPaymentMethodBuilder {
    card: Option<PaymentPaymentMethodCard>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    payment_method_type: Option<PaymentMethodTypes>,
}

impl PaymentPaymentMethodBuilder {
    pub fn card(mut self, value: PaymentPaymentMethodCard) -> Self {
        self.card = Some(value);
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

    pub fn payment_method_type(mut self, value: PaymentMethodTypes) -> Self {
        self.payment_method_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentPaymentMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](PaymentPaymentMethodBuilder::created_at)
    /// - [`id`](PaymentPaymentMethodBuilder::id)
    /// - [`payment_method_type`](PaymentPaymentMethodBuilder::payment_method_type)
    pub fn build(self) -> Result<PaymentPaymentMethod, BuildError> {
        Ok(PaymentPaymentMethod {
            card: self.card,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            payment_method_type: self
                .payment_method_type
                .ok_or_else(|| BuildError::missing_field("payment_method_type"))?,
        })
    }
}
