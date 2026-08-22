pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Shipment {
    /// The account that owns this shipment, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// The shipping carrier detected for this shipment. Null until a tracking update identifies it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(default)]
    pub checkpoints: Vec<ShipmentCheckpoint>,
    /// The datetime the shipment was created (ISO 8601).
    #[serde(default)]
    pub created_at: String,
    /// Shipment ID, prefixed `ship_`.
    #[serde(default)]
    pub id: String,
    /// The payment this shipment fulfills, prefixed `pay_`.
    #[serde(default)]
    pub payment_id: String,
    /// The current delivery status of this shipment.
    pub status: ShipmentStatus,
    /// The carrier-assigned tracking number used to look up shipment progress.
    #[serde(default)]
    pub tracking_number: String,
    /// A customer-facing URL to track this shipment's progress.
    #[serde(default)]
    pub tracking_url: String,
    /// The datetime the shipment was last updated (ISO 8601).
    #[serde(default)]
    pub updated_at: String,
}

impl Shipment {
    pub fn builder() -> ShipmentBuilder {
        <ShipmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShipmentBuilder {
    account_id: Option<String>,
    carrier: Option<String>,
    checkpoints: Option<Vec<ShipmentCheckpoint>>,
    created_at: Option<String>,
    id: Option<String>,
    payment_id: Option<String>,
    status: Option<ShipmentStatus>,
    tracking_number: Option<String>,
    tracking_url: Option<String>,
    updated_at: Option<String>,
}

impl ShipmentBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn carrier(mut self, value: impl Into<String>) -> Self {
        self.carrier = Some(value.into());
        self
    }

    pub fn checkpoints(mut self, value: Vec<ShipmentCheckpoint>) -> Self {
        self.checkpoints = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ShipmentStatus) -> Self {
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

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Shipment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](ShipmentBuilder::account_id)
    /// - [`checkpoints`](ShipmentBuilder::checkpoints)
    /// - [`created_at`](ShipmentBuilder::created_at)
    /// - [`id`](ShipmentBuilder::id)
    /// - [`payment_id`](ShipmentBuilder::payment_id)
    /// - [`status`](ShipmentBuilder::status)
    /// - [`tracking_number`](ShipmentBuilder::tracking_number)
    /// - [`tracking_url`](ShipmentBuilder::tracking_url)
    /// - [`updated_at`](ShipmentBuilder::updated_at)
    pub fn build(self) -> Result<Shipment, BuildError> {
        Ok(Shipment {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            carrier: self.carrier,
            checkpoints: self
                .checkpoints
                .ok_or_else(|| BuildError::missing_field("checkpoints"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            payment_id: self
                .payment_id
                .ok_or_else(|| BuildError::missing_field("payment_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            tracking_number: self
                .tracking_number
                .ok_or_else(|| BuildError::missing_field("tracking_number"))?,
            tracking_url: self
                .tracking_url
                .ok_or_else(|| BuildError::missing_field("tracking_url"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
