pub use crate::prelude::*;

/// Use `running` to start a run of a ready recommendation, `executed` to record that it was carried out, `incomplete` to record that a running recommendation's run ended without carrying it out, `superseded` to reject a ready recommendation, or `acknowledged` to mark an executed run as seen; the recommendation stays `executed` and records `acknowledged_at`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateEconomicIntelligenceRequestStatus {
    Running,
    Executed,
    Incomplete,
    Superseded,
    Acknowledged,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateEconomicIntelligenceRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Running => serializer.serialize_str("running"),
            Self::Executed => serializer.serialize_str("executed"),
            Self::Incomplete => serializer.serialize_str("incomplete"),
            Self::Superseded => serializer.serialize_str("superseded"),
            Self::Acknowledged => serializer.serialize_str("acknowledged"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateEconomicIntelligenceRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "running" => Ok(Self::Running),
            "executed" => Ok(Self::Executed),
            "incomplete" => Ok(Self::Incomplete),
            "superseded" => Ok(Self::Superseded),
            "acknowledged" => Ok(Self::Acknowledged),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateEconomicIntelligenceRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Running => write!(f, "running"),
            Self::Executed => write!(f, "executed"),
            Self::Incomplete => write!(f, "incomplete"),
            Self::Superseded => write!(f, "superseded"),
            Self::Acknowledged => write!(f, "acknowledged"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
