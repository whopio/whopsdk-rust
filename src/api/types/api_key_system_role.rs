pub use crate::prelude::*;

/// System role the key inherits its permissions from, or `null` when it uses an explicit permissions policy. Only account API keys can use a system role.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApiKeySystemRole {
    Owner,
    Admin,
    Moderator,
    SalesManager,
    Advertiser,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ApiKeySystemRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Owner => serializer.serialize_str("owner"),
            Self::Admin => serializer.serialize_str("admin"),
            Self::Moderator => serializer.serialize_str("moderator"),
            Self::SalesManager => serializer.serialize_str("sales_manager"),
            Self::Advertiser => serializer.serialize_str("advertiser"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ApiKeySystemRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "owner" => Ok(Self::Owner),
            "admin" => Ok(Self::Admin),
            "moderator" => Ok(Self::Moderator),
            "sales_manager" => Ok(Self::SalesManager),
            "advertiser" => Ok(Self::Advertiser),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ApiKeySystemRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Owner => write!(f, "owner"),
            Self::Admin => write!(f, "admin"),
            Self::Moderator => write!(f, "moderator"),
            Self::SalesManager => write!(f, "sales_manager"),
            Self::Advertiser => write!(f, "advertiser"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
