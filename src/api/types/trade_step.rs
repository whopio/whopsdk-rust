pub use crate::prelude::*;

/// What a live trade is doing now: `depositing` while the wallet's USDT0 is being sent to the bridge, `landing` once sent and until it reaches the trading balance, `placing` while the order or close is with the venue, `withdrawing` while USDC heads back to the wallet, `returning` while money an unfilled order bridged in goes back. A spot order is `quoting` while the swap is priced, `swapping` while it runs onchain, and `crediting` until the new balance shows in the wallet. `null` once the trade is no longer pending or processing.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TradeStep {
    Depositing,
    Landing,
    Placing,
    Withdrawing,
    Returning,
    Quoting,
    Swapping,
    Crediting,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TradeStep {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Depositing => serializer.serialize_str("depositing"),
            Self::Landing => serializer.serialize_str("landing"),
            Self::Placing => serializer.serialize_str("placing"),
            Self::Withdrawing => serializer.serialize_str("withdrawing"),
            Self::Returning => serializer.serialize_str("returning"),
            Self::Quoting => serializer.serialize_str("quoting"),
            Self::Swapping => serializer.serialize_str("swapping"),
            Self::Crediting => serializer.serialize_str("crediting"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TradeStep {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "depositing" => Ok(Self::Depositing),
            "landing" => Ok(Self::Landing),
            "placing" => Ok(Self::Placing),
            "withdrawing" => Ok(Self::Withdrawing),
            "returning" => Ok(Self::Returning),
            "quoting" => Ok(Self::Quoting),
            "swapping" => Ok(Self::Swapping),
            "crediting" => Ok(Self::Crediting),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TradeStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Depositing => write!(f, "depositing"),
            Self::Landing => write!(f, "landing"),
            Self::Placing => write!(f, "placing"),
            Self::Withdrawing => write!(f, "withdrawing"),
            Self::Returning => write!(f, "returning"),
            Self::Quoting => write!(f, "quoting"),
            Self::Swapping => write!(f, "swapping"),
            Self::Crediting => write!(f, "crediting"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
