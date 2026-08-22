pub use crate::prelude::*;

/// The source of an AI chat message
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AiChatMessageSourceTypes {
    Manual,
    Suggestion,
    Link,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AiChatMessageSourceTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Manual => serializer.serialize_str("manual"),
            Self::Suggestion => serializer.serialize_str("suggestion"),
            Self::Link => serializer.serialize_str("link"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AiChatMessageSourceTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "manual" => Ok(Self::Manual),
            "suggestion" => Ok(Self::Suggestion),
            "link" => Ok(Self::Link),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AiChatMessageSourceTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::Suggestion => write!(f, "suggestion"),
            Self::Link => write!(f, "link"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
