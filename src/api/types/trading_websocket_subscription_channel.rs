pub use crate::prelude::*;

/// The live update stream this subscription opens.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TradingWebsocketSubscriptionChannel {
    ClearinghouseState,
    OpenOrders,
    OrderUpdates,
    UserFills,
    UserEvents,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TradingWebsocketSubscriptionChannel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ClearinghouseState => serializer.serialize_str("clearinghouse_state"),
            Self::OpenOrders => serializer.serialize_str("open_orders"),
            Self::OrderUpdates => serializer.serialize_str("order_updates"),
            Self::UserFills => serializer.serialize_str("user_fills"),
            Self::UserEvents => serializer.serialize_str("user_events"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TradingWebsocketSubscriptionChannel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "clearinghouse_state" => Ok(Self::ClearinghouseState),
            "open_orders" => Ok(Self::OpenOrders),
            "order_updates" => Ok(Self::OrderUpdates),
            "user_fills" => Ok(Self::UserFills),
            "user_events" => Ok(Self::UserEvents),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TradingWebsocketSubscriptionChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClearinghouseState => write!(f, "clearinghouse_state"),
            Self::OpenOrders => write!(f, "open_orders"),
            Self::OrderUpdates => write!(f, "order_updates"),
            Self::UserFills => write!(f, "user_fills"),
            Self::UserEvents => write!(f, "user_events"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
