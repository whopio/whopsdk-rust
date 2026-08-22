pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListSocialAccountsRequestPlatform {
    X,
    Instagram,
    Youtube,
    Tiktok,
    Facebook,
    Discord,
    Telegram,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListSocialAccountsRequestPlatform {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::X => serializer.serialize_str("x"),
            Self::Instagram => serializer.serialize_str("instagram"),
            Self::Youtube => serializer.serialize_str("youtube"),
            Self::Tiktok => serializer.serialize_str("tiktok"),
            Self::Facebook => serializer.serialize_str("facebook"),
            Self::Discord => serializer.serialize_str("discord"),
            Self::Telegram => serializer.serialize_str("telegram"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListSocialAccountsRequestPlatform {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "x" => Ok(Self::X),
            "instagram" => Ok(Self::Instagram),
            "youtube" => Ok(Self::Youtube),
            "tiktok" => Ok(Self::Tiktok),
            "facebook" => Ok(Self::Facebook),
            "discord" => Ok(Self::Discord),
            "telegram" => Ok(Self::Telegram),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListSocialAccountsRequestPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(f, "x"),
            Self::Instagram => write!(f, "instagram"),
            Self::Youtube => write!(f, "youtube"),
            Self::Tiktok => write!(f, "tiktok"),
            Self::Facebook => write!(f, "facebook"),
            Self::Discord => write!(f, "discord"),
            Self::Telegram => write!(f, "telegram"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
