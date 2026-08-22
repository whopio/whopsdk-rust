pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostChatReactionCreatedPayloadDataAudienceType {
    Channel,
    Users,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostChatReactionCreatedPayloadDataAudienceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Channel => serializer.serialize_str("channel"),
            Self::Users => serializer.serialize_str("users"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostChatReactionCreatedPayloadDataAudienceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "channel" => Ok(Self::Channel),
            "users" => Ok(Self::Users),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostChatReactionCreatedPayloadDataAudienceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Channel => write!(f, "channel"),
            Self::Users => write!(f, "users"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
