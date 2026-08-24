pub use crate::prelude::*;

/// The available bounty statuses to choose from.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statuses {
    Published,
    Archived,
    Scheduled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Statuses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Published => serializer.serialize_str("published"),
            Self::Archived => serializer.serialize_str("archived"),
            Self::Scheduled => serializer.serialize_str("scheduled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Statuses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "published" => Ok(Self::Published),
            "archived" => Ok(Self::Archived),
            "scheduled" => Ok(Self::Scheduled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Statuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Published => write!(f, "published"),
            Self::Archived => write!(f, "archived"),
            Self::Scheduled => write!(f, "scheduled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
