pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMembersRequestOrder {
    CreatedAt,
    JoinedAt,
    LastAccessedAt,
    LastActivityAt,
    MostRecentActionAt,
    UsdTotalSpent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListMembersRequestOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::JoinedAt => serializer.serialize_str("joined_at"),
            Self::LastAccessedAt => serializer.serialize_str("last_accessed_at"),
            Self::LastActivityAt => serializer.serialize_str("last_activity_at"),
            Self::MostRecentActionAt => serializer.serialize_str("most_recent_action_at"),
            Self::UsdTotalSpent => serializer.serialize_str("usd_total_spent"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListMembersRequestOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "joined_at" => Ok(Self::JoinedAt),
            "last_accessed_at" => Ok(Self::LastAccessedAt),
            "last_activity_at" => Ok(Self::LastActivityAt),
            "most_recent_action_at" => Ok(Self::MostRecentActionAt),
            "usd_total_spent" => Ok(Self::UsdTotalSpent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListMembersRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::JoinedAt => write!(f, "joined_at"),
            Self::LastAccessedAt => write!(f, "last_accessed_at"),
            Self::LastActivityAt => write!(f, "last_activity_at"),
            Self::MostRecentActionAt => write!(f, "most_recent_action_at"),
            Self::UsdTotalSpent => write!(f, "usd_total_spent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
