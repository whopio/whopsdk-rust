pub use crate::prelude::*;

/// A payment transaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentFinancingTransactionsItem {
    /// The amount of the payment transaction.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The date and time the payment transaction was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The unique identifier for the payment transaction.
    #[serde(default)]
    pub id: String,
    /// The status of the payment transaction.
    pub status: PaymentTransactionStatuses,
    /// The type of the payment transaction.
    pub transaction_type: PaymentTransactionTypes,
}

impl PaymentFinancingTransactionsItem {
    pub fn builder() -> PaymentFinancingTransactionsItemBuilder {
        <PaymentFinancingTransactionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentFinancingTransactionsItemBuilder {
    amount: Option<f64>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    status: Option<PaymentTransactionStatuses>,
    transaction_type: Option<PaymentTransactionTypes>,
}

impl PaymentFinancingTransactionsItemBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
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

    pub fn status(mut self, value: PaymentTransactionStatuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn transaction_type(mut self, value: PaymentTransactionTypes) -> Self {
        self.transaction_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentFinancingTransactionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PaymentFinancingTransactionsItemBuilder::amount)
    /// - [`created_at`](PaymentFinancingTransactionsItemBuilder::created_at)
    /// - [`id`](PaymentFinancingTransactionsItemBuilder::id)
    /// - [`status`](PaymentFinancingTransactionsItemBuilder::status)
    /// - [`transaction_type`](PaymentFinancingTransactionsItemBuilder::transaction_type)
    pub fn build(self) -> Result<PaymentFinancingTransactionsItem, BuildError> {
        Ok(PaymentFinancingTransactionsItem {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            transaction_type: self
                .transaction_type
                .ok_or_else(|| BuildError::missing_field("transaction_type"))?,
        })
    }
}
