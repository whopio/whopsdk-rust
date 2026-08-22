pub use crate::prelude::*;

/// The type of embed for a lesson
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EmbedTypes {
    Youtube,
    Loom,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EmbedTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Youtube => serializer.serialize_str("youtube"),
            Self::Loom => serializer.serialize_str("loom"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EmbedTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "youtube" => Ok(Self::Youtube),
            "loom" => Ok(Self::Loom),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EmbedTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Youtube => write!(f, "youtube"),
            Self::Loom => write!(f, "loom"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
