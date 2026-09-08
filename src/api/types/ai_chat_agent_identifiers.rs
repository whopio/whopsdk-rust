pub use crate::prelude::*;

/// The AI agent that handles an AI chat.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AiChatAgentIdentifiers {
    General,
    Support,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AiChatAgentIdentifiers {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::General => serializer.serialize_str("general"),
            Self::Support => serializer.serialize_str("support"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AiChatAgentIdentifiers {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "general" => Ok(Self::General),
            "support" => Ok(Self::Support),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AiChatAgentIdentifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::General => write!(f, "general"),
            Self::Support => write!(f, "support"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
