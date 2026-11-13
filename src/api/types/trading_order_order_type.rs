pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TradingOrderOrderType {
    Limit,
    Market,
    TakeProfit,
    StopLoss,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TradingOrderOrderType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Limit => serializer.serialize_str("limit"),
            Self::Market => serializer.serialize_str("market"),
            Self::TakeProfit => serializer.serialize_str("take_profit"),
            Self::StopLoss => serializer.serialize_str("stop_loss"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TradingOrderOrderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "limit" => Ok(Self::Limit),
            "market" => Ok(Self::Market),
            "take_profit" => Ok(Self::TakeProfit),
            "stop_loss" => Ok(Self::StopLoss),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TradingOrderOrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Limit => write!(f, "limit"),
            Self::Market => write!(f, "market"),
            Self::TakeProfit => write!(f, "take_profit"),
            Self::StopLoss => write!(f, "stop_loss"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
