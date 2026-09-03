pub use crate::prelude::*;

/// The payment associated with this shipment. Null if the payment has been deleted or is inaccessible.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShipmentLegacyPayment {
    /// The unique identifier for the payment.
    #[serde(default)]
    pub id: String,
}

impl ShipmentLegacyPayment {
    pub fn builder() -> ShipmentLegacyPaymentBuilder {
        <ShipmentLegacyPaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShipmentLegacyPaymentBuilder {
    id: Option<String>,
}

impl ShipmentLegacyPaymentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ShipmentLegacyPayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ShipmentLegacyPaymentBuilder::id)
    pub fn build(self) -> Result<ShipmentLegacyPayment, BuildError> {
        Ok(ShipmentLegacyPayment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
