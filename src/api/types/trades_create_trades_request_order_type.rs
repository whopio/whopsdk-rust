pub use crate::prelude::*;

/// `market` fills now within `slippage_bps` of the mid price, and anything unfilled is canceled; `limit` fills at `price` or better, and how long it stays on the book follows `time_in_force`. `take_profit` and `stop_loss` reduce the open perpetual position with a market order once the price crosses `trigger_price`; the trade stays `processing` until then. Required for an order.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateTradesRequestOrderType {
    Market,
    Limit,
    TakeProfit,
    StopLoss,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateTradesRequestOrderType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Market => serializer.serialize_str("market"),
            Self::Limit => serializer.serialize_str("limit"),
            Self::TakeProfit => serializer.serialize_str("take_profit"),
            Self::StopLoss => serializer.serialize_str("stop_loss"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateTradesRequestOrderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "market" => Ok(Self::Market),
            "limit" => Ok(Self::Limit),
            "take_profit" => Ok(Self::TakeProfit),
            "stop_loss" => Ok(Self::StopLoss),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateTradesRequestOrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Market => write!(f, "market"),
            Self::Limit => write!(f, "limit"),
            Self::TakeProfit => write!(f, "take_profit"),
            Self::StopLoss => write!(f, "stop_loss"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
