pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TradingOrderStatus {
    Open,
    Filled,
    Canceled,
    Triggered,
    Rejected,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TradingOrderStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Open => serializer.serialize_str("open"),
            Self::Filled => serializer.serialize_str("filled"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Triggered => serializer.serialize_str("triggered"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TradingOrderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "open" => Ok(Self::Open),
            "filled" => Ok(Self::Filled),
            "canceled" => Ok(Self::Canceled),
            "triggered" => Ok(Self::Triggered),
            "rejected" => Ok(Self::Rejected),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TradingOrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Filled => write!(f, "filled"),
            Self::Canceled => write!(f, "canceled"),
            Self::Triggered => write!(f, "triggered"),
            Self::Rejected => write!(f, "rejected"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
