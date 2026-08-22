pub use crate::prelude::*;

/// `joined` while the member is part of the account, `left` after they leave.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MemberStatus {
    Joined,
    Left,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MemberStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Joined => serializer.serialize_str("joined"),
            Self::Left => serializer.serialize_str("left"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MemberStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "joined" => Ok(Self::Joined),
            "left" => Ok(Self::Left),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MemberStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Joined => write!(f, "joined"),
            Self::Left => write!(f, "left"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
