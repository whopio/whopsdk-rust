pub use crate::prelude::*;

/// Only for a perpetual limit order. `good_til_canceled` rests on the book until it fills; `add_liquidity_only` is canceled instead of filling at once; `immediate_or_cancel` fills what it can now and cancels the rest. Defaults to `good_til_canceled`. The trade stays `processing` while the order rests.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateTradesRequestTimeInForce {
    AddLiquidityOnly,
    GoodTilCanceled,
    ImmediateOrCancel,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateTradesRequestTimeInForce {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AddLiquidityOnly => serializer.serialize_str("add_liquidity_only"),
            Self::GoodTilCanceled => serializer.serialize_str("good_til_canceled"),
            Self::ImmediateOrCancel => serializer.serialize_str("immediate_or_cancel"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateTradesRequestTimeInForce {
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

impl fmt::Display for CreateTradesRequestTimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddLiquidityOnly => write!(f, "add_liquidity_only"),
            Self::GoodTilCanceled => write!(f, "good_til_canceled"),
            Self::ImmediateOrCancel => write!(f, "immediate_or_cancel"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
