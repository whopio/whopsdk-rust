pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ShipmentCheckpoint {
    /// Where the carrier recorded the scan, such as `PHILADELPHIA, PA`. Null when the carrier sent none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Carrier's description of the scan, such as `Departed USPS Regional Facility`. Null when the carrier sent none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Delivery status this carrier scan maps to.
    pub status: ShipmentCheckpointStatus,
    /// When the carrier recorded the scan, as an ISO 8601 timestamp. Null when the carrier sent no scan time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl ShipmentCheckpoint {
    pub fn builder() -> ShipmentCheckpointBuilder {
        <ShipmentCheckpointBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShipmentCheckpointBuilder {
    location: Option<String>,
    message: Option<String>,
    status: Option<ShipmentCheckpointStatus>,
    timestamp: Option<String>,
}

impl ShipmentCheckpointBuilder {
    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn status(mut self, value: ShipmentCheckpointStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn timestamp(mut self, value: impl Into<String>) -> Self {
        self.timestamp = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ShipmentCheckpoint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ShipmentCheckpointBuilder::status)
    pub fn build(self) -> Result<ShipmentCheckpoint, BuildError> {
        Ok(ShipmentCheckpoint {
            location: self.location,
            message: self.message,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            timestamp: self.timestamp,
        })
    }
}
