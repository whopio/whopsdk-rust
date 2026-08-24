pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SummaryDisputesRequestGroupsItem {
    Status,
    Currency,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SummaryDisputesRequestGroupsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Status => serializer.serialize_str("status"),
            Self::Currency => serializer.serialize_str("currency"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SummaryDisputesRequestGroupsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "status" => Ok(Self::Status),
            "currency" => Ok(Self::Currency),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SummaryDisputesRequestGroupsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status => write!(f, "status"),
            Self::Currency => write!(f, "currency"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
