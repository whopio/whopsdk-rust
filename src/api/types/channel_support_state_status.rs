pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChannelSupportStateStatus {
    Unresolved,
    Resolved,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ChannelSupportStateStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unresolved => serializer.serialize_str("unresolved"),
            Self::Resolved => serializer.serialize_str("resolved"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ChannelSupportStateStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unresolved" => Ok(Self::Unresolved),
            "resolved" => Ok(Self::Resolved),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ChannelSupportStateStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unresolved => write!(f, "unresolved"),
            Self::Resolved => write!(f, "resolved"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
