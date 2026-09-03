pub use crate::prelude::*;

/// A dispute alert represents an early warning notification from a payment processor about a potential dispute or chargeback.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisputeAlertLegacy {
    /// The type of the dispute alert.
    pub alert_type: DisputeAlertTypes,
    /// The alerted amount in the specified currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// Whether this alert incurs a charge.
    #[serde(default)]
    pub charge_for_alert: bool,
    /// The time the dispute alert was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The three-letter ISO currency code for the alerted amount.
    pub currency: Currencies,
    /// The dispute associated with the dispute alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute: Option<DisputeAlertLegacyDispute>,
    /// The unique identifier of the dispute alert.
    #[serde(default)]
    pub id: String,
    /// The payment associated with the dispute alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<DisputeAlertLegacyPayment>,
    /// The date of the original transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub transaction_date: Option<DateTime<FixedOffset>>,
}

impl DisputeAlertLegacy {
    pub fn builder() -> DisputeAlertLegacyBuilder {
        <DisputeAlertLegacyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeAlertLegacyBuilder {
    alert_type: Option<DisputeAlertTypes>,
    amount: Option<f64>,
    charge_for_alert: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    dispute: Option<DisputeAlertLegacyDispute>,
    id: Option<String>,
    payment: Option<DisputeAlertLegacyPayment>,
    transaction_date: Option<DateTime<FixedOffset>>,
}

impl DisputeAlertLegacyBuilder {
    pub fn alert_type(mut self, value: DisputeAlertTypes) -> Self {
        self.alert_type = Some(value);
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn charge_for_alert(mut self, value: bool) -> Self {
        self.charge_for_alert = Some(value);
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

    pub fn dispute(mut self, value: DisputeAlertLegacyDispute) -> Self {
        self.dispute = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn payment(mut self, value: DisputeAlertLegacyPayment) -> Self {
        self.payment = Some(value);
        self
    }

    pub fn transaction_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.transaction_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisputeAlertLegacy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`alert_type`](DisputeAlertLegacyBuilder::alert_type)
    /// - [`amount`](DisputeAlertLegacyBuilder::amount)
    /// - [`charge_for_alert`](DisputeAlertLegacyBuilder::charge_for_alert)
    /// - [`created_at`](DisputeAlertLegacyBuilder::created_at)
    /// - [`currency`](DisputeAlertLegacyBuilder::currency)
    /// - [`id`](DisputeAlertLegacyBuilder::id)
    pub fn build(self) -> Result<DisputeAlertLegacy, BuildError> {
        Ok(DisputeAlertLegacy {
            alert_type: self
                .alert_type
                .ok_or_else(|| BuildError::missing_field("alert_type"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            charge_for_alert: self
                .charge_for_alert
                .ok_or_else(|| BuildError::missing_field("charge_for_alert"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            dispute: self.dispute,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            payment: self.payment,
            transaction_date: self.transaction_date,
        })
    }
}
