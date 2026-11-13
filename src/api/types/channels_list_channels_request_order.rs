pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListChannelsRequestOrder {
    CreatedAt,
    LastMessageAt,
    LastViewedAt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListChannelsRequestOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::LastMessageAt => serializer.serialize_str("last_message_at"),
            Self::LastViewedAt => serializer.serialize_str("last_viewed_at"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListChannelsRequestOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "last_message_at" => Ok(Self::LastMessageAt),
            "last_viewed_at" => Ok(Self::LastViewedAt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListChannelsRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::LastMessageAt => write!(f, "last_message_at"),
            Self::LastViewedAt => write!(f, "last_viewed_at"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
