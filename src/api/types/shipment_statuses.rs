pub use crate::prelude::*;

/// The status of a shipment
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShipmentStatuses {
    Unknown,
    PreTransit,
    InTransit,
    OutForDelivery,
    Delivered,
    AvailableForPickup,
    ReturnToSender,
    Failure,
    Cancelled,
    Error,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ShipmentStatuses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::PreTransit => serializer.serialize_str("pre_transit"),
            Self::InTransit => serializer.serialize_str("in_transit"),
            Self::OutForDelivery => serializer.serialize_str("out_for_delivery"),
            Self::Delivered => serializer.serialize_str("delivered"),
            Self::AvailableForPickup => serializer.serialize_str("available_for_pickup"),
            Self::ReturnToSender => serializer.serialize_str("return_to_sender"),
            Self::Failure => serializer.serialize_str("failure"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::Error => serializer.serialize_str("error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ShipmentStatuses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unknown" => Ok(Self::Unknown),
            "pre_transit" => Ok(Self::PreTransit),
            "in_transit" => Ok(Self::InTransit),
            "out_for_delivery" => Ok(Self::OutForDelivery),
            "delivered" => Ok(Self::Delivered),
            "available_for_pickup" => Ok(Self::AvailableForPickup),
            "return_to_sender" => Ok(Self::ReturnToSender),
            "failure" => Ok(Self::Failure),
            "cancelled" => Ok(Self::Cancelled),
            "error" => Ok(Self::Error),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ShipmentStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::PreTransit => write!(f, "pre_transit"),
            Self::InTransit => write!(f, "in_transit"),
            Self::OutForDelivery => write!(f, "out_for_delivery"),
            Self::Delivered => write!(f, "delivered"),
            Self::AvailableForPickup => write!(f, "available_for_pickup"),
            Self::ReturnToSender => write!(f, "return_to_sender"),
            Self::Failure => write!(f, "failure"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Error => write!(f, "error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
