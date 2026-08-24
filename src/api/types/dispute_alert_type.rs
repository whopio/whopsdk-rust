pub use crate::prelude::*;

/// What the issuer sent. `early_fraud_warning` is a fraud report on a settled payment (Visa TC40 / Mastercard SAFE) — refunding still avoids the chargeback, and Whop never charges a fee for one. `dispute_alert` is a pre-dispute notice from the issuer's alert network, which Whop pays for and passes on as a fee. `rapid_dispute_resolution` is a Visa RDR case the network already closed by refunding the payment — nothing is left to act on.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisputeAlertType {
    EarlyFraudWarning,
    DisputeAlert,
    RapidDisputeResolution,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DisputeAlertType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EarlyFraudWarning => serializer.serialize_str("early_fraud_warning"),
            Self::DisputeAlert => serializer.serialize_str("dispute_alert"),
            Self::RapidDisputeResolution => serializer.serialize_str("rapid_dispute_resolution"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DisputeAlertType {
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

impl fmt::Display for DisputeAlertType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EarlyFraudWarning => write!(f, "early_fraud_warning"),
            Self::DisputeAlert => write!(f, "dispute_alert"),
            Self::RapidDisputeResolution => write!(f, "rapid_dispute_resolution"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
