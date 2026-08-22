pub use crate::prelude::*;

/// A refund represents a full or partial reversal of a payment, including the amount, status, and payment provider.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentRefundsItem {
    /// The refunded amount as a decimal in the specified currency, such as 10.43 for $10.43 USD.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The datetime the refund was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The three-letter ISO currency code for the refunded amount.
    pub currency: Currencies,
    /// The unique identifier for the refund.
    #[serde(default)]
    pub id: String,
    /// The current processing status of the refund, such as pending, succeeded, or failed.
    pub status: RefundStatuses,
}

impl PaymentRefundsItem {
    pub fn builder() -> PaymentRefundsItemBuilder {
        <PaymentRefundsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentRefundsItemBuilder {
    amount: Option<f64>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    id: Option<String>,
    status: Option<RefundStatuses>,
}

impl PaymentRefundsItemBuilder {
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: RefundStatuses) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentRefundsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PaymentRefundsItemBuilder::amount)
    /// - [`created_at`](PaymentRefundsItemBuilder::created_at)
    /// - [`currency`](PaymentRefundsItemBuilder::currency)
    /// - [`id`](PaymentRefundsItemBuilder::id)
    /// - [`status`](PaymentRefundsItemBuilder::status)
    pub fn build(self) -> Result<PaymentRefundsItem, BuildError> {
        Ok(PaymentRefundsItem {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
