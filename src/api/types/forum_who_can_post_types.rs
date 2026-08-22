pub use crate::prelude::*;

/// Who can post on a forum feed
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForumWhoCanPostTypes {
    Everyone,
    Admins,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ForumWhoCanPostTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Everyone => serializer.serialize_str("everyone"),
            Self::Admins => serializer.serialize_str("admins"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ForumWhoCanPostTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "everyone" => Ok(Self::Everyone),
            "admins" => Ok(Self::Admins),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ForumWhoCanPostTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Everyone => write!(f, "everyone"),
            Self::Admins => write!(f, "admins"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
