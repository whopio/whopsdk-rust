pub use crate::prelude::*;

/// The payment associated with the dispute alert.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeAlertListItemPayment {
    /// The unique identifier for the payment.
    #[serde(default)]
    pub id: String,
}

impl DisputeAlertListItemPayment {
    pub fn builder() -> DisputeAlertListItemPaymentBuilder {
        <DisputeAlertListItemPaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeAlertListItemPaymentBuilder {
    id: Option<String>,
}

impl DisputeAlertListItemPaymentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeAlertListItemPayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DisputeAlertListItemPaymentBuilder::id)
    pub fn build(self) -> Result<DisputeAlertListItemPayment, BuildError> {
        Ok(DisputeAlertListItemPayment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
