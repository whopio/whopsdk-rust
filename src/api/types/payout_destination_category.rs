pub use crate::prelude::*;

/// The category of a payout destination.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PayoutDestinationCategory {
    Crypto,
    Rtp,
    NextDayBank,
    BankWire,
    DigitalWallet,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PayoutDestinationCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Crypto => serializer.serialize_str("crypto"),
            Self::Rtp => serializer.serialize_str("rtp"),
            Self::NextDayBank => serializer.serialize_str("next_day_bank"),
            Self::BankWire => serializer.serialize_str("bank_wire"),
            Self::DigitalWallet => serializer.serialize_str("digital_wallet"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PayoutDestinationCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "crypto" => Ok(Self::Crypto),
            "rtp" => Ok(Self::Rtp),
            "next_day_bank" => Ok(Self::NextDayBank),
            "bank_wire" => Ok(Self::BankWire),
            "digital_wallet" => Ok(Self::DigitalWallet),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PayoutDestinationCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Crypto => write!(f, "crypto"),
            Self::Rtp => write!(f, "rtp"),
            Self::NextDayBank => write!(f, "next_day_bank"),
            Self::BankWire => write!(f, "bank_wire"),
            Self::DigitalWallet => write!(f, "digital_wallet"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
