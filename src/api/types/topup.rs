pub use crate::prelude::*;

/// A payment represents a completed or attempted charge. Payments track the amount, status, currency, and payment method used.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Topup {
    /// The datetime the payment was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The three-letter ISO currency code for this payment (e.g., 'usd', 'eur').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currencies>,
    /// If the payment failed, the reason for the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// The unique identifier for the payment.
    #[serde(default)]
    pub id: String,
    /// The time at which this payment was successfully collected. Null if the payment has not yet succeeded. As a Unix timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub paid_at: Option<DateTime<FixedOffset>>,
    /// The current lifecycle state of this payment (e.g., 'draft', 'open', 'paid', 'void').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiptStatus>,
    /// The total to show to the creator (excluding buyer fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total: Option<f64>,
}

impl Topup {
    pub fn builder() -> TopupBuilder {
        <TopupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TopupBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    failure_message: Option<String>,
    id: Option<String>,
    paid_at: Option<DateTime<FixedOffset>>,
    status: Option<ReceiptStatus>,
    total: Option<f64>,
}

impl TopupBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn failure_message(mut self, value: impl Into<String>) -> Self {
        self.failure_message = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn paid_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.paid_at = Some(value);
        self
    }

    pub fn status(mut self, value: ReceiptStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn total(mut self, value: f64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Topup`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](TopupBuilder::created_at)
    /// - [`id`](TopupBuilder::id)
    pub fn build(self) -> Result<Topup, BuildError> {
        Ok(Topup {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self.currency,
            failure_message: self.failure_message,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            paid_at: self.paid_at,
            status: self.status,
            total: self.total,
        })
    }
}
