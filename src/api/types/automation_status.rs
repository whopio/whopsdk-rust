pub use crate::prelude::*;

/// `active` enrolls members and runs their steps. `closed` enrolls no one new, and members already in it finish. `stopped` enrolls no one, and its members' enrollments have ended. A new automation is `stopped` until its first publish makes it `active`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutomationStatus {
    Active,
    Closed,
    Stopped,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AutomationStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Closed => serializer.serialize_str("closed"),
            Self::Stopped => serializer.serialize_str("stopped"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AutomationStatus {
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

impl fmt::Display for AutomationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Closed => write!(f, "closed"),
            Self::Stopped => write!(f, "stopped"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
