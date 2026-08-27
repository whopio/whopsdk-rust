pub use crate::prelude::*;

/// Why pending funds without a settlement date aren't moving yet, when it's something the merchant can act on. `null` when there's no reason to show (still clearing, or the account is held for a reason that isn't merchant-actionable).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountPaymentControlsUndatedPendingReason {
    KycIncomplete,
    PendingInformationRequest,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountPaymentControlsUndatedPendingReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::KycIncomplete => serializer.serialize_str("kyc_incomplete"),
            Self::PendingInformationRequest => {
                serializer.serialize_str("pending_information_request")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountPaymentControlsUndatedPendingReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "kyc_incomplete" => Ok(Self::KycIncomplete),
            "pending_information_request" => Ok(Self::PendingInformationRequest),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountPaymentControlsUndatedPendingReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KycIncomplete => write!(f, "kyc_incomplete"),
            Self::PendingInformationRequest => write!(f, "pending_information_request"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
