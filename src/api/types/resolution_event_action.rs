pub use crate::prelude::*;

/// The action recorded in this event.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResolutionEventAction {
    Created,
    Responded,
    Accepted,
    Denied,
    Appealed,
    Withdrew,
    RequestedMoreInfo,
    Escalated,
    DisputeOpened,
    DisputeCustomerWon,
    DisputeMerchantWon,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResolutionEventAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Created => serializer.serialize_str("created"),
            Self::Responded => serializer.serialize_str("responded"),
            Self::Accepted => serializer.serialize_str("accepted"),
            Self::Denied => serializer.serialize_str("denied"),
            Self::Appealed => serializer.serialize_str("appealed"),
            Self::Withdrew => serializer.serialize_str("withdrew"),
            Self::RequestedMoreInfo => serializer.serialize_str("requested_more_info"),
            Self::Escalated => serializer.serialize_str("escalated"),
            Self::DisputeOpened => serializer.serialize_str("dispute_opened"),
            Self::DisputeCustomerWon => serializer.serialize_str("dispute_customer_won"),
            Self::DisputeMerchantWon => serializer.serialize_str("dispute_merchant_won"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResolutionEventAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created" => Ok(Self::Created),
            "responded" => Ok(Self::Responded),
            "accepted" => Ok(Self::Accepted),
            "denied" => Ok(Self::Denied),
            "appealed" => Ok(Self::Appealed),
            "withdrew" => Ok(Self::Withdrew),
            "requested_more_info" => Ok(Self::RequestedMoreInfo),
            "escalated" => Ok(Self::Escalated),
            "dispute_opened" => Ok(Self::DisputeOpened),
            "dispute_customer_won" => Ok(Self::DisputeCustomerWon),
            "dispute_merchant_won" => Ok(Self::DisputeMerchantWon),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResolutionEventAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Created => write!(f, "created"),
            Self::Responded => write!(f, "responded"),
            Self::Accepted => write!(f, "accepted"),
            Self::Denied => write!(f, "denied"),
            Self::Appealed => write!(f, "appealed"),
            Self::Withdrew => write!(f, "withdrew"),
            Self::RequestedMoreInfo => write!(f, "requested_more_info"),
            Self::Escalated => write!(f, "escalated"),
            Self::DisputeOpened => write!(f, "dispute_opened"),
            Self::DisputeCustomerWon => write!(f, "dispute_customer_won"),
            Self::DisputeMerchantWon => write!(f, "dispute_merchant_won"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
