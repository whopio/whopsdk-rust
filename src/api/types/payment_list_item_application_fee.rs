pub use crate::prelude::*;

/// The application fee charged on this payment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentListItemApplicationFee {
    /// The application fee amount.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The amount of the application fee that has been captured.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount_captured: f64,
    /// The amount of the application fee that has been refunded.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount_refunded: f64,
    /// The datetime the application fee was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The currency of the application fee.
    pub currency: Currencies,
    /// The unique identifier for the application fee.
    #[serde(default)]
    pub id: String,
}

impl PaymentListItemApplicationFee {
    pub fn builder() -> PaymentListItemApplicationFeeBuilder {
        <PaymentListItemApplicationFeeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentListItemApplicationFeeBuilder {
    amount: Option<f64>,
    amount_captured: Option<f64>,
    amount_refunded: Option<f64>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    id: Option<String>,
}

impl PaymentListItemApplicationFeeBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn amount_captured(mut self, value: f64) -> Self {
        self.amount_captured = Some(value);
        self
    }

    pub fn amount_refunded(mut self, value: f64) -> Self {
        self.amount_refunded = Some(value);
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentListItemApplicationFee`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PaymentListItemApplicationFeeBuilder::amount)
    /// - [`amount_captured`](PaymentListItemApplicationFeeBuilder::amount_captured)
    /// - [`amount_refunded`](PaymentListItemApplicationFeeBuilder::amount_refunded)
    /// - [`created_at`](PaymentListItemApplicationFeeBuilder::created_at)
    /// - [`currency`](PaymentListItemApplicationFeeBuilder::currency)
    /// - [`id`](PaymentListItemApplicationFeeBuilder::id)
    pub fn build(self) -> Result<PaymentListItemApplicationFee, BuildError> {
        Ok(PaymentListItemApplicationFee {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            amount_captured: self
                .amount_captured
                .ok_or_else(|| BuildError::missing_field("amount_captured"))?,
            amount_refunded: self
                .amount_refunded
                .ok_or_else(|| BuildError::missing_field("amount_refunded"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
