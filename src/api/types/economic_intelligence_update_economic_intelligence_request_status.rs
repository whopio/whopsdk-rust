pub use crate::prelude::*;

/// Use `executed` after approval to start the action, or `superseded` to reject it.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateEconomicIntelligenceRequestStatus {
    Executed,
    Superseded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateEconomicIntelligenceRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Executed => serializer.serialize_str("executed"),
            Self::Superseded => serializer.serialize_str("superseded"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateEconomicIntelligenceRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "executed" => Ok(Self::Executed),
            "superseded" => Ok(Self::Superseded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateEconomicIntelligenceRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Executed => write!(f, "executed"),
            Self::Superseded => write!(f, "superseded"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
