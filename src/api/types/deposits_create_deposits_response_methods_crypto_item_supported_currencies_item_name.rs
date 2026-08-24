pub use crate::prelude::*;

/// Token symbol.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItemName {
    Arb,
    Bnb,
    Eth,
    Eurc,
    Hype,
    Pyusd,
    Sol,
    Usd1,
    Usdc,
    UsdcE,
    Usdg,
    Usdt,
    Usdt0,
    UsDe,
    UsDm,
    Xo,
    Xpl,
    PUsd,
    WEth,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItemName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Arb => serializer.serialize_str("ARB"),
            Self::Bnb => serializer.serialize_str("BNB"),
            Self::Eth => serializer.serialize_str("ETH"),
            Self::Eurc => serializer.serialize_str("EURC"),
            Self::Hype => serializer.serialize_str("HYPE"),
            Self::Pyusd => serializer.serialize_str("PYUSD"),
            Self::Sol => serializer.serialize_str("SOL"),
            Self::Usd1 => serializer.serialize_str("USD1"),
            Self::Usdc => serializer.serialize_str("USDC"),
            Self::UsdcE => serializer.serialize_str("USDC.e"),
            Self::Usdg => serializer.serialize_str("USDG"),
            Self::Usdt => serializer.serialize_str("USDT"),
            Self::Usdt0 => serializer.serialize_str("USDT0"),
            Self::UsDe => serializer.serialize_str("USDe"),
            Self::UsDm => serializer.serialize_str("USDm"),
            Self::Xo => serializer.serialize_str("XO"),
            Self::Xpl => serializer.serialize_str("XPL"),
            Self::PUsd => serializer.serialize_str("pUSD"),
            Self::WEth => serializer.serialize_str("wETH"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItemName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ARB" => Ok(Self::Arb),
            "BNB" => Ok(Self::Bnb),
            "ETH" => Ok(Self::Eth),
            "EURC" => Ok(Self::Eurc),
            "HYPE" => Ok(Self::Hype),
            "PYUSD" => Ok(Self::Pyusd),
            "SOL" => Ok(Self::Sol),
            "USD1" => Ok(Self::Usd1),
            "USDC" => Ok(Self::Usdc),
            "USDC.e" => Ok(Self::UsdcE),
            "USDG" => Ok(Self::Usdg),
            "USDT" => Ok(Self::Usdt),
            "USDT0" => Ok(Self::Usdt0),
            "USDe" => Ok(Self::UsDe),
            "USDm" => Ok(Self::UsDm),
            "XO" => Ok(Self::Xo),
            "XPL" => Ok(Self::Xpl),
            "pUSD" => Ok(Self::PUsd),
            "wETH" => Ok(Self::WEth),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItemName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Arb => write!(f, "ARB"),
            Self::Bnb => write!(f, "BNB"),
            Self::Eth => write!(f, "ETH"),
            Self::Eurc => write!(f, "EURC"),
            Self::Hype => write!(f, "HYPE"),
            Self::Pyusd => write!(f, "PYUSD"),
            Self::Sol => write!(f, "SOL"),
            Self::Usd1 => write!(f, "USD1"),
            Self::Usdc => write!(f, "USDC"),
            Self::UsdcE => write!(f, "USDC.e"),
            Self::Usdg => write!(f, "USDG"),
            Self::Usdt => write!(f, "USDT"),
            Self::Usdt0 => write!(f, "USDT0"),
            Self::UsDe => write!(f, "USDe"),
            Self::UsDm => write!(f, "USDm"),
            Self::Xo => write!(f, "XO"),
            Self::Xpl => write!(f, "XPL"),
            Self::PUsd => write!(f, "pUSD"),
            Self::WEth => write!(f, "wETH"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
