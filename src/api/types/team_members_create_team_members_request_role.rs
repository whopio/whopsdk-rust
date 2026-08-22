pub use crate::prelude::*;

/// The system role to grant.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateTeamMembersRequestRole {
    Owner,
    Admin,
    SalesManager,
    Moderator,
    Advertiser,
    Workforce,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateTeamMembersRequestRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Owner => serializer.serialize_str("owner"),
            Self::Admin => serializer.serialize_str("admin"),
            Self::SalesManager => serializer.serialize_str("sales_manager"),
            Self::Moderator => serializer.serialize_str("moderator"),
            Self::Advertiser => serializer.serialize_str("advertiser"),
            Self::Workforce => serializer.serialize_str("workforce"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateTeamMembersRequestRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "owner" => Ok(Self::Owner),
            "admin" => Ok(Self::Admin),
            "sales_manager" => Ok(Self::SalesManager),
            "moderator" => Ok(Self::Moderator),
            "advertiser" => Ok(Self::Advertiser),
            "workforce" => Ok(Self::Workforce),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateTeamMembersRequestRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Owner => write!(f, "owner"),
            Self::Admin => write!(f, "admin"),
            Self::SalesManager => write!(f, "sales_manager"),
            Self::Moderator => write!(f, "moderator"),
            Self::Advertiser => write!(f, "advertiser"),
            Self::Workforce => write!(f, "workforce"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
