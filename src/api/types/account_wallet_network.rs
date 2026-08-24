pub use crate::prelude::*;

/// The blockchain network the wallet lives on
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountWalletNetwork {
    Solana,
    Ethereum,
    Bitcoin,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountWalletNetwork {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Solana => serializer.serialize_str("solana"),
            Self::Ethereum => serializer.serialize_str("ethereum"),
            Self::Bitcoin => serializer.serialize_str("bitcoin"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountWalletNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "solana" => Ok(Self::Solana),
            "ethereum" => Ok(Self::Ethereum),
            "bitcoin" => Ok(Self::Bitcoin),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountWalletNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Solana => write!(f, "solana"),
            Self::Ethereum => write!(f, "ethereum"),
            Self::Bitcoin => write!(f, "bitcoin"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
