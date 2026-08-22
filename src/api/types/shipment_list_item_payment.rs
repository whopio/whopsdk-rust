pub use crate::prelude::*;

/// The payment associated with this shipment. Null if the payment has been deleted or is inaccessible.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShipmentListItemPayment {
    /// The unique identifier for the payment.
    #[serde(default)]
    pub id: String,
}

impl ShipmentListItemPayment {
    pub fn builder() -> ShipmentListItemPaymentBuilder {
        <ShipmentListItemPaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShipmentListItemPaymentBuilder {
    id: Option<String>,
}

impl ShipmentListItemPaymentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ShipmentListItemPayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ShipmentListItemPaymentBuilder::id)
    pub fn build(self) -> Result<ShipmentListItemPayment, BuildError> {
        Ok(ShipmentListItemPayment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
