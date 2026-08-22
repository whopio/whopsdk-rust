pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListBountiesRequestStatus {
    Scheduled,
    Open,
    Closed,
    Completed,
    Canceled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListBountiesRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Scheduled => serializer.serialize_str("scheduled"),
            Self::Open => serializer.serialize_str("open"),
            Self::Closed => serializer.serialize_str("closed"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListBountiesRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "scheduled" => Ok(Self::Scheduled),
            "open" => Ok(Self::Open),
            "closed" => Ok(Self::Closed),
            "completed" => Ok(Self::Completed),
            "canceled" => Ok(Self::Canceled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListBountiesRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scheduled => write!(f, "scheduled"),
            Self::Open => write!(f, "open"),
            Self::Closed => write!(f, "closed"),
            Self::Completed => write!(f, "completed"),
            Self::Canceled => write!(f, "canceled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
