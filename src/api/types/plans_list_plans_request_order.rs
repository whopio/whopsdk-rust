pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListPlansRequestOrder {
    Id,
    ActiveMembersCount,
    CreatedAt,
    InternalNotes,
    ExpirationDays,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListPlansRequestOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Id => serializer.serialize_str("id"),
            Self::ActiveMembersCount => serializer.serialize_str("active_members_count"),
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::InternalNotes => serializer.serialize_str("internal_notes"),
            Self::ExpirationDays => serializer.serialize_str("expiration_days"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListPlansRequestOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "id" => Ok(Self::Id),
            "active_members_count" => Ok(Self::ActiveMembersCount),
            "created_at" => Ok(Self::CreatedAt),
            "internal_notes" => Ok(Self::InternalNotes),
            "expiration_days" => Ok(Self::ExpirationDays),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListPlansRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::ActiveMembersCount => write!(f, "active_members_count"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::InternalNotes => write!(f, "internal_notes"),
            Self::ExpirationDays => write!(f, "expiration_days"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
