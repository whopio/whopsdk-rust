pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListApiLogsRequestStatus {
    Success,
    Error,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListApiLogsRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Success => serializer.serialize_str("success"),
            Self::Error => serializer.serialize_str("error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListApiLogsRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "success" => Ok(Self::Success),
            "error" => Ok(Self::Error),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListApiLogsRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::Error => write!(f, "error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
