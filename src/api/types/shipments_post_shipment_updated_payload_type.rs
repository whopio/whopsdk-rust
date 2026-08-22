pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostShipmentUpdatedPayloadType {
    #[serde(rename = "shipment.updated")]
    ShipmentUpdated,
}
impl fmt::Display for PostShipmentUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ShipmentUpdated => "shipment.updated",
        };
        write!(f, "{}", s)
    }
}
