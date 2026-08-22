pub use crate::prelude::*;

/// A physical shipment associated with a payment, including carrier details and tracking information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ShipmentListItem {
    /// The datetime the shipment was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The estimated delivery date for this shipment. Null if the carrier has not provided an estimate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub delivery_estimate: Option<DateTime<FixedOffset>>,
    /// The unique identifier for the shipment.
    #[serde(default)]
    pub id: String,
    /// The payment associated with this shipment. Null if the payment has been deleted or is inaccessible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<ShipmentListItemPayment>,
    /// The shipping service level used for this shipment. Null if the carrier does not specify a service tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// The current delivery status of this shipment.
    pub status: ShipmentStatuses,
    /// A more granular status providing additional detail about the shipment's current state. Null if no substatus applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substatus: Option<ShipmentSubstatuses>,
    /// The carrier-assigned tracking number used to look up shipment progress.
    #[serde(default)]
    pub tracking_code: String,
    /// The datetime the shipment was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ShipmentListItem {
    pub fn builder() -> ShipmentListItemBuilder {
        <ShipmentListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShipmentListItemBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    delivery_estimate: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    payment: Option<ShipmentListItemPayment>,
    service: Option<String>,
    status: Option<ShipmentStatuses>,
    substatus: Option<ShipmentSubstatuses>,
    tracking_code: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ShipmentListItemBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn delivery_estimate(mut self, value: DateTime<FixedOffset>) -> Self {
        self.delivery_estimate = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn payment(mut self, value: ShipmentListItemPayment) -> Self {
        self.payment = Some(value);
        self
    }

    pub fn service(mut self, value: impl Into<String>) -> Self {
        self.service = Some(value.into());
        self
    }

    pub fn status(mut self, value: ShipmentStatuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn substatus(mut self, value: ShipmentSubstatuses) -> Self {
        self.substatus = Some(value);
        self
    }

    pub fn tracking_code(mut self, value: impl Into<String>) -> Self {
        self.tracking_code = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ShipmentListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ShipmentListItemBuilder::created_at)
    /// - [`id`](ShipmentListItemBuilder::id)
    /// - [`status`](ShipmentListItemBuilder::status)
    /// - [`tracking_code`](ShipmentListItemBuilder::tracking_code)
    /// - [`updated_at`](ShipmentListItemBuilder::updated_at)
    pub fn build(self) -> Result<ShipmentListItem, BuildError> {
        Ok(ShipmentListItem {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            delivery_estimate: self.delivery_estimate,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            payment: self.payment,
            service: self.service,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            substatus: self.substatus,
            tracking_code: self
                .tracking_code
                .ok_or_else(|| BuildError::missing_field("tracking_code"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
