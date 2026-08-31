pub use crate::prelude::*;

/// The platform to connect the social account on. Use `meta_business` to connect Meta Business assets, which is how Facebook Pages and Instagram accounts are connected — there is no separate `instagram` value. Use `tiktok` for TikTok accounts.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectSocialAccountsRequestPlatform {
    MetaBusiness,
    Tiktok,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConnectSocialAccountsRequestPlatform {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MetaBusiness => serializer.serialize_str("meta_business"),
            Self::Tiktok => serializer.serialize_str("tiktok"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConnectSocialAccountsRequestPlatform {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "meta_business" => Ok(Self::MetaBusiness),
            "tiktok" => Ok(Self::Tiktok),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConnectSocialAccountsRequestPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MetaBusiness => write!(f, "meta_business"),
            Self::Tiktok => write!(f, "tiktok"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
