pub use crate::prelude::*;

/// The status of an entry to a waitlist.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntryStatus {
    Drafted,
    Pending,
    Approved,
    Denied,
    Any,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EntryStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Drafted => serializer.serialize_str("drafted"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Approved => serializer.serialize_str("approved"),
            Self::Denied => serializer.serialize_str("denied"),
            Self::Any => serializer.serialize_str("any"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EntryStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "drafted" => Ok(Self::Drafted),
            "pending" => Ok(Self::Pending),
            "approved" => Ok(Self::Approved),
            "denied" => Ok(Self::Denied),
            "any" => Ok(Self::Any),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EntryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Drafted => write!(f, "drafted"),
            Self::Pending => write!(f, "pending"),
            Self::Approved => write!(f, "approved"),
            Self::Denied => write!(f, "denied"),
            Self::Any => write!(f, "any"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
