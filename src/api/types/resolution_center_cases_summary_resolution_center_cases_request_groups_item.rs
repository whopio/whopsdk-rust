pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SummaryResolutionCenterCasesRequestGroupsItem {
    Status,
    Reason,
    Outcome,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SummaryResolutionCenterCasesRequestGroupsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Status => serializer.serialize_str("status"),
            Self::Reason => serializer.serialize_str("reason"),
            Self::Outcome => serializer.serialize_str("outcome"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SummaryResolutionCenterCasesRequestGroupsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "status" => Ok(Self::Status),
            "reason" => Ok(Self::Reason),
            "outcome" => Ok(Self::Outcome),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SummaryResolutionCenterCasesRequestGroupsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status => write!(f, "status"),
            Self::Reason => write!(f, "reason"),
            Self::Outcome => write!(f, "outcome"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
