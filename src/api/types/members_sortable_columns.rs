pub use crate::prelude::*;

/// Which columns can be used to sort.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MembersSortableColumns {
    Id,
    UsdTotalSpent,
    CreatedAt,
    JoinedAt,
    MostRecentAction,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MembersSortableColumns {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Id => serializer.serialize_str("id"),
            Self::UsdTotalSpent => serializer.serialize_str("usd_total_spent"),
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::JoinedAt => serializer.serialize_str("joined_at"),
            Self::MostRecentAction => serializer.serialize_str("most_recent_action"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MembersSortableColumns {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "id" => Ok(Self::Id),
            "usd_total_spent" => Ok(Self::UsdTotalSpent),
            "created_at" => Ok(Self::CreatedAt),
            "joined_at" => Ok(Self::JoinedAt),
            "most_recent_action" => Ok(Self::MostRecentAction),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MembersSortableColumns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::UsdTotalSpent => write!(f, "usd_total_spent"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::JoinedAt => write!(f, "joined_at"),
            Self::MostRecentAction => write!(f, "most_recent_action"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
