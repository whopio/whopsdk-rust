pub use crate::prelude::*;

/// `joined` for accepted members, `pending` while the invite is pending.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TeamMemberStatus {
    Joined,
    Pending,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TeamMemberStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Joined => serializer.serialize_str("joined"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TeamMemberStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "joined" => Ok(Self::Joined),
            "pending" => Ok(Self::Pending),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TeamMemberStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Joined => write!(f, "joined"),
            Self::Pending => write!(f, "pending"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
