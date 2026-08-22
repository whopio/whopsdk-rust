pub use crate::prelude::*;

/// The status of a withdrawal request
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WithdrawalStatus {
    Requested,
    AwaitingPayment,
    InTransit,
    Completed,
    Failed,
    Canceled,
    Denied,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WithdrawalStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Requested => serializer.serialize_str("requested"),
            Self::AwaitingPayment => serializer.serialize_str("awaiting_payment"),
            Self::InTransit => serializer.serialize_str("in_transit"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Denied => serializer.serialize_str("denied"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WithdrawalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "requested" => Ok(Self::Requested),
            "awaiting_payment" => Ok(Self::AwaitingPayment),
            "in_transit" => Ok(Self::InTransit),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "canceled" => Ok(Self::Canceled),
            "denied" => Ok(Self::Denied),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WithdrawalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Requested => write!(f, "requested"),
            Self::AwaitingPayment => write!(f, "awaiting_payment"),
            Self::InTransit => write!(f, "in_transit"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Canceled => write!(f, "canceled"),
            Self::Denied => write!(f, "denied"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
