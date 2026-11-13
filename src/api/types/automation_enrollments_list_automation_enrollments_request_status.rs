pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListAutomationEnrollmentsRequestStatus {
    Active,
    Completed,
    Canceled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListAutomationEnrollmentsRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListAutomationEnrollmentsRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "completed" => Ok(Self::Completed),
            "canceled" => Ok(Self::Canceled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListAutomationEnrollmentsRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Completed => write!(f, "completed"),
            Self::Canceled => write!(f, "canceled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
