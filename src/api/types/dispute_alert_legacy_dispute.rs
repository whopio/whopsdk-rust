pub use crate::prelude::*;

/// The dispute associated with the dispute alert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisputeAlertLegacyDispute {
    /// The disputed amount in the specified currency, formatted as a decimal.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The datetime the dispute was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The three-letter ISO currency code for the disputed amount.
    pub currency: Currencies,
    /// The unique identifier for the dispute.
    #[serde(default)]
    pub id: String,
    /// A human-readable reason for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The current status of the dispute lifecycle, such as needs_response, under_review, won, or lost.
    pub status: DisputeStatuses,
}

impl DisputeAlertLegacyDispute {
    pub fn builder() -> DisputeAlertLegacyDisputeBuilder {
        <DisputeAlertLegacyDisputeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeAlertLegacyDisputeBuilder {
    amount: Option<f64>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    id: Option<String>,
    reason: Option<String>,
    status: Option<DisputeStatuses>,
}

impl DisputeAlertLegacyDisputeBuilder {
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

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn status(mut self, value: DisputeStatuses) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisputeAlertLegacyDispute`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](DisputeAlertLegacyDisputeBuilder::amount)
    /// - [`currency`](DisputeAlertLegacyDisputeBuilder::currency)
    /// - [`id`](DisputeAlertLegacyDisputeBuilder::id)
    /// - [`status`](DisputeAlertLegacyDisputeBuilder::status)
    pub fn build(self) -> Result<DisputeAlertLegacyDispute, BuildError> {
        Ok(DisputeAlertLegacyDispute {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            created_at: self.created_at,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self.reason,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
