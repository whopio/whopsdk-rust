pub use crate::prelude::*;

/// The payment record that is the subject of this resolution case.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResolutionCenterCaseLegacyPayment {
    /// The datetime the payment was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The three-letter ISO currency code for this payment (e.g., 'usd', 'eur').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currencies>,
    /// The unique identifier for the payment.
    #[serde(default)]
    pub id: String,
    /// The time at which this payment was successfully collected. Null if the payment has not yet succeeded. As a Unix timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub paid_at: Option<DateTime<FixedOffset>>,
    /// The payment amount before taxes and discounts are applied. In the currency specified by the currency field.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub subtotal: Option<f64>,
    /// The total amount charged to the customer for this payment, including taxes and after any discounts. In the currency specified by the currency field.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total: f64,
}

impl ResolutionCenterCaseLegacyPayment {
    pub fn builder() -> ResolutionCenterCaseLegacyPaymentBuilder {
        <ResolutionCenterCaseLegacyPaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionCenterCaseLegacyPaymentBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    id: Option<String>,
    paid_at: Option<DateTime<FixedOffset>>,
    subtotal: Option<f64>,
    total: Option<f64>,
}

impl ResolutionCenterCaseLegacyPaymentBuilder {
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

    pub fn paid_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.paid_at = Some(value);
        self
    }

    pub fn subtotal(mut self, value: f64) -> Self {
        self.subtotal = Some(value);
        self
    }

    pub fn total(mut self, value: f64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResolutionCenterCaseLegacyPayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ResolutionCenterCaseLegacyPaymentBuilder::created_at)
    /// - [`id`](ResolutionCenterCaseLegacyPaymentBuilder::id)
    /// - [`total`](ResolutionCenterCaseLegacyPaymentBuilder::total)
    pub fn build(self) -> Result<ResolutionCenterCaseLegacyPayment, BuildError> {
        Ok(ResolutionCenterCaseLegacyPayment {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self.currency,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            paid_at: self.paid_at,
            subtotal: self.subtotal,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
