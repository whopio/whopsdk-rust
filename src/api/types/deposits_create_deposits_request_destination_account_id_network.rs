pub use crate::prelude::*;

/// Destination wallet network.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateDepositsRequestDestinationAccountIdNetwork {
    Ethereum,
    Polygon,
    Base,
    Solana,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateDepositsRequestDestinationAccountIdNetwork {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ethereum => serializer.serialize_str("ethereum"),
            Self::Polygon => serializer.serialize_str("polygon"),
            Self::Base => serializer.serialize_str("base"),
            Self::Solana => serializer.serialize_str("solana"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateDepositsRequestDestinationAccountIdNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ethereum" => Ok(Self::Ethereum),
            "polygon" => Ok(Self::Polygon),
            "base" => Ok(Self::Base),
            "solana" => Ok(Self::Solana),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateDepositsRequestDestinationAccountIdNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ethereum => write!(f, "ethereum"),
            Self::Polygon => write!(f, "polygon"),
            Self::Base => write!(f, "base"),
            Self::Solana => write!(f, "solana"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
