pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostShipmentCreatedPayloadType {
    #[serde(rename = "shipment.created")]
    ShipmentCreated,
}
impl fmt::Display for PostShipmentCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ShipmentCreated => "shipment.created",
        };
        write!(f, "{}", s)
    }
}
