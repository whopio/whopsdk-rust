pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentHold {
    /// The amount currently held, in the hold's currency.
    #[serde(default)]
    pub amount: Money,
    /// The reserve percentage recorded when the hold was created, for example 3.5 for 3.5%. Null for other hold types or when no percentage was recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub percentage: Option<f64>,
    /// When the held funds are scheduled to become available, as an ISO 8601 timestamp. Never earlier than the payment's settlement date. Null when release depends on an event, such as shipment resolution, rather than a date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_at: Option<String>,
    /// The reason funds are held: `reserve`, `bnpl`, `sequra`, `fraud_hold`, or `preshipment_hold`.
    pub r#type: PaymentHoldType,
}

impl PaymentHold {
    pub fn builder() -> PaymentHoldBuilder {
        <PaymentHoldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentHoldBuilder {
    amount: Option<Money>,
    percentage: Option<f64>,
    release_at: Option<String>,
    r#type: Option<PaymentHoldType>,
}

impl PaymentHoldBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    pub fn release_at(mut self, value: impl Into<String>) -> Self {
        self.release_at = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PaymentHoldType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentHold`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PaymentHoldBuilder::amount)
    /// - [`r#type`](PaymentHoldBuilder::r#type)
    pub fn build(self) -> Result<PaymentHold, BuildError> {
        Ok(PaymentHold {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            percentage: self.percentage,
            release_at: self.release_at,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
