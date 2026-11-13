pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListAutomationsRequestStatus {
    Active,
    Closed,
    Stopped,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListAutomationsRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Closed => serializer.serialize_str("closed"),
            Self::Stopped => serializer.serialize_str("stopped"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListAutomationsRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "closed" => Ok(Self::Closed),
            "stopped" => Ok(Self::Stopped),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListAutomationsRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Closed => write!(f, "closed"),
            Self::Stopped => write!(f, "stopped"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
