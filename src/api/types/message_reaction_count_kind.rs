pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MessageReactionCountKind {
    Like,
    View,
    Emoji,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MessageReactionCountKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Like => serializer.serialize_str("like"),
            Self::View => serializer.serialize_str("view"),
            Self::Emoji => serializer.serialize_str("emoji"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MessageReactionCountKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "like" => Ok(Self::Like),
            "view" => Ok(Self::View),
            "emoji" => Ok(Self::Emoji),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MessageReactionCountKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Like => write!(f, "like"),
            Self::View => write!(f, "view"),
            Self::Emoji => write!(f, "emoji"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
