pub use crate::prelude::*;

/// Why refunding can no longer avoid a chargeback. `network_resolved` when a Visa RDR already closed the case, `payment_unmatched` when no payment matched, `payment_not_captured` when it never captured money, `payment_disputed` once the payment carries a dispute, `payment_refunded` once fully refunded. `null` while `actionable` is true.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisputeAlertNotActionableReason {
    NetworkResolved,
    PaymentUnmatched,
    PaymentNotCaptured,
    PaymentDisputed,
    PaymentRefunded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DisputeAlertNotActionableReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NetworkResolved => serializer.serialize_str("network_resolved"),
            Self::PaymentUnmatched => serializer.serialize_str("payment_unmatched"),
            Self::PaymentNotCaptured => serializer.serialize_str("payment_not_captured"),
            Self::PaymentDisputed => serializer.serialize_str("payment_disputed"),
            Self::PaymentRefunded => serializer.serialize_str("payment_refunded"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DisputeAlertNotActionableReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "network_resolved" => Ok(Self::NetworkResolved),
            "payment_unmatched" => Ok(Self::PaymentUnmatched),
            "payment_not_captured" => Ok(Self::PaymentNotCaptured),
            "payment_disputed" => Ok(Self::PaymentDisputed),
            "payment_refunded" => Ok(Self::PaymentRefunded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DisputeAlertNotActionableReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NetworkResolved => write!(f, "network_resolved"),
            Self::PaymentUnmatched => write!(f, "payment_unmatched"),
            Self::PaymentNotCaptured => write!(f, "payment_not_captured"),
            Self::PaymentDisputed => write!(f, "payment_disputed"),
            Self::PaymentRefunded => write!(f, "payment_refunded"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
