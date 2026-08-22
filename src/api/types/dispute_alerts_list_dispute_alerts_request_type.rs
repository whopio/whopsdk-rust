pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListDisputeAlertsRequestType {
    EarlyFraudWarning,
    DisputeAlert,
    RapidDisputeResolution,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListDisputeAlertsRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EarlyFraudWarning => serializer.serialize_str("early_fraud_warning"),
            Self::DisputeAlert => serializer.serialize_str("dispute_alert"),
            Self::RapidDisputeResolution => serializer.serialize_str("rapid_dispute_resolution"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListDisputeAlertsRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "early_fraud_warning" => Ok(Self::EarlyFraudWarning),
            "dispute_alert" => Ok(Self::DisputeAlert),
            "rapid_dispute_resolution" => Ok(Self::RapidDisputeResolution),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListDisputeAlertsRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EarlyFraudWarning => write!(f, "early_fraud_warning"),
            Self::DisputeAlert => write!(f, "dispute_alert"),
            Self::RapidDisputeResolution => write!(f, "rapid_dispute_resolution"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
