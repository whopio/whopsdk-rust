pub use crate::prelude::*;

/// The different statuses a Member can have.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MemberStatuses {
    Drafted,
    Joined,
    Left,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MemberStatuses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Drafted => serializer.serialize_str("drafted"),
            Self::Joined => serializer.serialize_str("joined"),
            Self::Left => serializer.serialize_str("left"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MemberStatuses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "drafted" => Ok(Self::Drafted),
            "joined" => Ok(Self::Joined),
            "left" => Ok(Self::Left),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MemberStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Drafted => write!(f, "drafted"),
            Self::Joined => write!(f, "joined"),
            Self::Left => write!(f, "left"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
