pub use crate::prelude::*;

/// Sort options for message channels
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MessageChannelOrder {
    CreatedAt,
    LastPostSentAt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MessageChannelOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::LastPostSentAt => serializer.serialize_str("last_post_sent_at"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MessageChannelOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "last_post_sent_at" => Ok(Self::LastPostSentAt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MessageChannelOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::LastPostSentAt => write!(f, "last_post_sent_at"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
