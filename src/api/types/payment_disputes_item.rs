pub use crate::prelude::*;

/// A dispute is a chargeback or payment challenge filed against a company, including evidence and response status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentDisputesItem {
    /// The disputed amount in the specified currency, formatted as a decimal.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The three-letter ISO currency code for the disputed amount.
    pub currency: Currencies,
    /// Whether the dispute evidence can still be edited and submitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    /// The unique identifier for the dispute.
    #[serde(default)]
    pub id: String,
    /// The deadline by which dispute evidence must be submitted. Null if no response deadline is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub needs_response_by: Option<DateTime<FixedOffset>>,
    /// Additional freeform notes submitted by the company as part of the dispute evidence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// A human-readable reason for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The current status of the dispute lifecycle, such as needs_response, under_review, won, or lost.
    pub status: DisputeStatuses,
}

impl PaymentDisputesItem {
    pub fn builder() -> PaymentDisputesItemBuilder {
        <PaymentDisputesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentDisputesItemBuilder {
    amount: Option<f64>,
    currency: Option<Currencies>,
    editable: Option<bool>,
    id: Option<String>,
    needs_response_by: Option<DateTime<FixedOffset>>,
    notes: Option<String>,
    reason: Option<String>,
    status: Option<DisputeStatuses>,
}

impl PaymentDisputesItemBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn editable(mut self, value: bool) -> Self {
        self.editable = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn needs_response_by(mut self, value: DateTime<FixedOffset>) -> Self {
        self.needs_response_by = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn status(mut self, value: DisputeStatuses) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentDisputesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PaymentDisputesItemBuilder::amount)
    /// - [`currency`](PaymentDisputesItemBuilder::currency)
    /// - [`id`](PaymentDisputesItemBuilder::id)
    /// - [`status`](PaymentDisputesItemBuilder::status)
    pub fn build(self) -> Result<PaymentDisputesItem, BuildError> {
        Ok(PaymentDisputesItem {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            editable: self.editable,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            needs_response_by: self.needs_response_by,
            notes: self.notes,
            reason: self.reason,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
