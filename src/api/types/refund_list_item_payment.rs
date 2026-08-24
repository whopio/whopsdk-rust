pub use crate::prelude::*;

/// The original payment that this refund was issued against. Null if the payment is no longer available.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundListItemPayment {
    /// The unique identifier for the payment.
    #[serde(default)]
    pub id: String,
}

impl RefundListItemPayment {
    pub fn builder() -> RefundListItemPaymentBuilder {
        <RefundListItemPaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundListItemPaymentBuilder {
    id: Option<String>,
}

impl RefundListItemPaymentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RefundListItemPayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RefundListItemPaymentBuilder::id)
    pub fn build(self) -> Result<RefundListItemPayment, BuildError> {
        Ok(RefundListItemPayment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
