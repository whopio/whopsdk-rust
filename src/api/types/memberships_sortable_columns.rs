pub use crate::prelude::*;

/// Which columns can be used to sort.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MembershipsSortableColumns {
    Id,
    CreatedAt,
    Status,
    CanceledAt,
    DateJoined,
    TotalSpend,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MembershipsSortableColumns {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Id => serializer.serialize_str("id"),
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::Status => serializer.serialize_str("status"),
            Self::CanceledAt => serializer.serialize_str("canceled_at"),
            Self::DateJoined => serializer.serialize_str("date_joined"),
            Self::TotalSpend => serializer.serialize_str("total_spend"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MembershipsSortableColumns {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "id" => Ok(Self::Id),
            "created_at" => Ok(Self::CreatedAt),
            "status" => Ok(Self::Status),
            "canceled_at" => Ok(Self::CanceledAt),
            "date_joined" => Ok(Self::DateJoined),
            "total_spend" => Ok(Self::TotalSpend),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MembershipsSortableColumns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::Status => write!(f, "status"),
            Self::CanceledAt => write!(f, "canceled_at"),
            Self::DateJoined => write!(f, "date_joined"),
            Self::TotalSpend => write!(f, "total_spend"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
