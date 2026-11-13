pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShipmentLookup {
    /// The matching payment, prefixed `pay_`.
    #[serde(default)]
    pub payment_id: String,
    /// The payment's latest shipment, or `null` when a paid physical purchase has no tracking yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment: Option<Shipment>,
}

impl ShipmentLookup {
    pub fn builder() -> ShipmentLookupBuilder {
        <ShipmentLookupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShipmentLookupBuilder {
    payment_id: Option<String>,
    shipment: Option<Shipment>,
}

impl ShipmentLookupBuilder {
    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn shipment(mut self, value: Shipment) -> Self {
        self.shipment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ShipmentLookup`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payment_id`](ShipmentLookupBuilder::payment_id)
    pub fn build(self) -> Result<ShipmentLookup, BuildError> {
        Ok(ShipmentLookup {
            payment_id: self
                .payment_id
                .ok_or_else(|| BuildError::missing_field("payment_id"))?,
            shipment: self.shipment,
        })
    }
}
