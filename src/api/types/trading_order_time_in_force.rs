pub use crate::prelude::*;

/// How long the order stays active. `null` when the provider omits it or reports a policy outside the supported values.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TradingOrderTimeInForce {
    AddLiquidityOnly,
    GoodTilCanceled,
    ImmediateOrCancel,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TradingOrderTimeInForce {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AddLiquidityOnly => serializer.serialize_str("add_liquidity_only"),
            Self::GoodTilCanceled => serializer.serialize_str("good_til_canceled"),
            Self::ImmediateOrCancel => serializer.serialize_str("immediate_or_cancel"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TradingOrderTimeInForce {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "add_liquidity_only" => Ok(Self::AddLiquidityOnly),
            "good_til_canceled" => Ok(Self::GoodTilCanceled),
            "immediate_or_cancel" => Ok(Self::ImmediateOrCancel),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TradingOrderTimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddLiquidityOnly => write!(f, "add_liquidity_only"),
            Self::GoodTilCanceled => write!(f, "good_til_canceled"),
            Self::ImmediateOrCancel => write!(f, "immediate_or_cancel"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
