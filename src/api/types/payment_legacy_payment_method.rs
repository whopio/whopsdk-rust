pub use crate::prelude::*;

/// The tokenized payment method reference used for this payment. Null if no token was used.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentLegacyPaymentMethod {
    /// The card data associated with the payment method, if its a debit or credit card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentLegacyPaymentMethodCard>,
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

impl PaymentLegacyPaymentMethod {
    pub fn builder() -> PaymentLegacyPaymentMethodBuilder {
        <PaymentLegacyPaymentMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentLegacyPaymentMethodBuilder {
    card: Option<PaymentLegacyPaymentMethodCard>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    payment_method_type: Option<PaymentMethodTypes>,
}

impl PaymentLegacyPaymentMethodBuilder {
    pub fn card(mut self, value: PaymentLegacyPaymentMethodCard) -> Self {
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

    /// Consumes the builder and constructs a [`PaymentLegacyPaymentMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](PaymentLegacyPaymentMethodBuilder::created_at)
    /// - [`id`](PaymentLegacyPaymentMethodBuilder::id)
    /// - [`payment_method_type`](PaymentLegacyPaymentMethodBuilder::payment_method_type)
    pub fn build(self) -> Result<PaymentLegacyPaymentMethod, BuildError> {
        Ok(PaymentLegacyPaymentMethod {
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
