pub use crate::prelude::*;

/// The different speeds of withdrawals
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WithdrawalSpeeds {
    Standard,
    Instant,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WithdrawalSpeeds {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Standard => serializer.serialize_str("standard"),
            Self::Instant => serializer.serialize_str("instant"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WithdrawalSpeeds {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "standard" => Ok(Self::Standard),
            "instant" => Ok(Self::Instant),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WithdrawalSpeeds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "standard"),
            Self::Instant => write!(f, "instant"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
