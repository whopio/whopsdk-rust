pub use crate::prelude::*;

/// The platform the member's account is on.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MembershipExternalAccountPlatform {
    Discord,
    Telegram,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MembershipExternalAccountPlatform {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Discord => serializer.serialize_str("discord"),
            Self::Telegram => serializer.serialize_str("telegram"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MembershipExternalAccountPlatform {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "discord" => Ok(Self::Discord),
            "telegram" => Ok(Self::Telegram),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MembershipExternalAccountPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Discord => write!(f, "discord"),
            Self::Telegram => write!(f, "telegram"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
