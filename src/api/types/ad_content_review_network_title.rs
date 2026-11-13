pub use crate::prelude::*;

/// The network. `whop` is Whop's own content policy, which every other network also requires.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdContentReviewNetworkTitle {
    Whop,
    Meta,
    Tiktok,
    Google,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdContentReviewNetworkTitle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Whop => serializer.serialize_str("whop"),
            Self::Meta => serializer.serialize_str("meta"),
            Self::Tiktok => serializer.serialize_str("tiktok"),
            Self::Google => serializer.serialize_str("google"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdContentReviewNetworkTitle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "whop" => Ok(Self::Whop),
            "meta" => Ok(Self::Meta),
            "tiktok" => Ok(Self::Tiktok),
            "google" => Ok(Self::Google),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdContentReviewNetworkTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Whop => write!(f, "whop"),
            Self::Meta => write!(f, "meta"),
            Self::Tiktok => write!(f, "tiktok"),
            Self::Google => write!(f, "google"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
