pub use crate::prelude::*;

/// The shipment attached to this payment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentShipment {
    /// The shipping carrier detected for this shipment. Null until a tracking update identifies it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// The unique identifier for the shipment.
    #[serde(default)]
    pub id: String,
    /// The current delivery status of this shipment.
    pub status: ShipmentStatuses,
    /// The carrier-assigned tracking number used to look up shipment progress.
    #[serde(default)]
    pub tracking_number: String,
    /// A customer-facing URL to track this shipment's progress.
    #[serde(default)]
    pub tracking_url: String,
}

impl PaymentShipment {
    pub fn builder() -> PaymentShipmentBuilder {
        <PaymentShipmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentShipmentBuilder {
    carrier: Option<String>,
    id: Option<String>,
    status: Option<ShipmentStatuses>,
    tracking_number: Option<String>,
    tracking_url: Option<String>,
}

impl PaymentShipmentBuilder {
    pub fn carrier(mut self, value: impl Into<String>) -> Self {
        self.carrier = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ShipmentStatuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tracking_number(mut self, value: impl Into<String>) -> Self {
        self.tracking_number = Some(value.into());
        self
    }

    pub fn tracking_url(mut self, value: impl Into<String>) -> Self {
        self.tracking_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentShipment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PaymentShipmentBuilder::id)
    /// - [`status`](PaymentShipmentBuilder::status)
    /// - [`tracking_number`](PaymentShipmentBuilder::tracking_number)
    /// - [`tracking_url`](PaymentShipmentBuilder::tracking_url)
    pub fn build(self) -> Result<PaymentShipment, BuildError> {
        Ok(PaymentShipment {
            carrier: self.carrier,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            tracking_number: self
                .tracking_number
                .ok_or_else(|| BuildError::missing_field("tracking_number"))?,
            tracking_url: self
                .tracking_url
                .ok_or_else(|| BuildError::missing_field("tracking_url"))?,
        })
    }
}
