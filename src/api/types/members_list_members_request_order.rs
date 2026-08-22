pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMembersRequestOrder {
    CreatedAt,
    JoinedAt,
    LastAccessedAt,
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
            Self::UsdTotalSpent => write!(f, "usd_total_spent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
