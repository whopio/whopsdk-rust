pub use crate::prelude::*;

/// Where the money is when the trade did not complete as asked: `wallet`, `trading_account`, or `unknown` while a person checks it. `null` otherwise.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TradeFundsLocation {
    Wallet,
    TradingAccount,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TradeFundsLocation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Wallet => serializer.serialize_str("wallet"),
            Self::TradingAccount => serializer.serialize_str("trading_account"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TradeFundsLocation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "wallet" => Ok(Self::Wallet),
            "trading_account" => Ok(Self::TradingAccount),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TradeFundsLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wallet => write!(f, "wallet"),
            Self::TradingAccount => write!(f, "trading_account"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
