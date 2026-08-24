pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostChatReactionCreatedPayloadDataChannelType {
    Chat,
    DirectMessage,
    Support,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostChatReactionCreatedPayloadDataChannelType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Chat => serializer.serialize_str("chat"),
            Self::DirectMessage => serializer.serialize_str("direct_message"),
            Self::Support => serializer.serialize_str("support"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostChatReactionCreatedPayloadDataChannelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "chat" => Ok(Self::Chat),
            "direct_message" => Ok(Self::DirectMessage),
            "support" => Ok(Self::Support),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostChatReactionCreatedPayloadDataChannelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Chat => write!(f, "chat"),
            Self::DirectMessage => write!(f, "direct_message"),
            Self::Support => write!(f, "support"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
