pub use crate::prelude::*;

/// A refund represents a full or partial reversal of a payment, including the amount, status, and payment provider.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefundListItem {
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
    /// The original payment that this refund was issued against. Null if the payment is no longer available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<RefundListItemPayment>,
    /// The payment provider that processed the refund.
    pub provider: PaymentProviders,
    /// The timestamp when the refund was created in the payment provider's system. Null if not available from the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub provider_created_at: Option<DateTime<FixedOffset>>,
    /// The availability status of the refund tracking reference from the payment processor. Null if no reference was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_status: Option<RefundReferenceStatuses>,
    /// The type of tracking reference provided by the payment processor, such as an acquirer reference number. Null if no reference was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<RefundReferenceTypes>,
    /// The tracking reference value from the payment processor, used to trace the refund through banking networks. Null if no reference was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value: Option<String>,
    /// The current processing status of the refund, such as pending, succeeded, or failed.
    pub status: RefundStatuses,
}

impl RefundListItem {
    pub fn builder() -> RefundListItemBuilder {
        <RefundListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundListItemBuilder {
    amount: Option<f64>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    id: Option<String>,
    payment: Option<RefundListItemPayment>,
    provider: Option<PaymentProviders>,
    provider_created_at: Option<DateTime<FixedOffset>>,
    reference_status: Option<RefundReferenceStatuses>,
    reference_type: Option<RefundReferenceTypes>,
    reference_value: Option<String>,
    status: Option<RefundStatuses>,
}

impl RefundListItemBuilder {
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

    pub fn payment(mut self, value: RefundListItemPayment) -> Self {
        self.payment = Some(value);
        self
    }

    pub fn provider(mut self, value: PaymentProviders) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn provider_created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.provider_created_at = Some(value);
        self
    }

    pub fn reference_status(mut self, value: RefundReferenceStatuses) -> Self {
        self.reference_status = Some(value);
        self
    }

    pub fn reference_type(mut self, value: RefundReferenceTypes) -> Self {
        self.reference_type = Some(value);
        self
    }

    pub fn reference_value(mut self, value: impl Into<String>) -> Self {
        self.reference_value = Some(value.into());
        self
    }

    pub fn status(mut self, value: RefundStatuses) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](RefundListItemBuilder::amount)
    /// - [`created_at`](RefundListItemBuilder::created_at)
    /// - [`currency`](RefundListItemBuilder::currency)
    /// - [`id`](RefundListItemBuilder::id)
    /// - [`provider`](RefundListItemBuilder::provider)
    /// - [`status`](RefundListItemBuilder::status)
    pub fn build(self) -> Result<RefundListItem, BuildError> {
        Ok(RefundListItem {
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
            payment: self.payment,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            provider_created_at: self.provider_created_at,
            reference_status: self.reference_status,
            reference_type: self.reference_type,
            reference_value: self.reference_value,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
