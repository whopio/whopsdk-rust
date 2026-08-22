pub use crate::prelude::*;

/// Network display name.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateDepositsResponseMethodsCryptoItemName {
    Ethereum,
    Solana,
    Base,
    BnbSmartChain,
    Hyperliquid,
    Hypercore,
    MegaEth,
    Polygon,
    Plasma,
    Arbitrum,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateDepositsResponseMethodsCryptoItemName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ethereum => serializer.serialize_str("Ethereum"),
            Self::Solana => serializer.serialize_str("Solana"),
            Self::Base => serializer.serialize_str("Base"),
            Self::BnbSmartChain => serializer.serialize_str("BNB Smart Chain"),
            Self::Hyperliquid => serializer.serialize_str("Hyperliquid"),
            Self::Hypercore => serializer.serialize_str("Hypercore"),
            Self::MegaEth => serializer.serialize_str("MegaETH"),
            Self::Polygon => serializer.serialize_str("Polygon"),
            Self::Plasma => serializer.serialize_str("Plasma"),
            Self::Arbitrum => serializer.serialize_str("Arbitrum"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateDepositsResponseMethodsCryptoItemName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Ethereum" => Ok(Self::Ethereum),
            "Solana" => Ok(Self::Solana),
            "Base" => Ok(Self::Base),
            "BNB Smart Chain" => Ok(Self::BnbSmartChain),
            "Hyperliquid" => Ok(Self::Hyperliquid),
            "Hypercore" => Ok(Self::Hypercore),
            "MegaETH" => Ok(Self::MegaEth),
            "Polygon" => Ok(Self::Polygon),
            "Plasma" => Ok(Self::Plasma),
            "Arbitrum" => Ok(Self::Arbitrum),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateDepositsResponseMethodsCryptoItemName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ethereum => write!(f, "Ethereum"),
            Self::Solana => write!(f, "Solana"),
            Self::Base => write!(f, "Base"),
            Self::BnbSmartChain => write!(f, "BNB Smart Chain"),
            Self::Hyperliquid => write!(f, "Hyperliquid"),
            Self::Hypercore => write!(f, "Hypercore"),
            Self::MegaEth => write!(f, "MegaETH"),
            Self::Polygon => write!(f, "Polygon"),
            Self::Plasma => write!(f, "Plasma"),
            Self::Arbitrum => write!(f, "Arbitrum"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
