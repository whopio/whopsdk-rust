pub use crate::prelude::*;

/// Why the refund was issued, when recorded.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
    ExpiredUncapturedCharge,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RefundReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Duplicate => serializer.serialize_str("duplicate"),
            Self::Fraudulent => serializer.serialize_str("fraudulent"),
            Self::RequestedByCustomer => serializer.serialize_str("requested_by_customer"),
            Self::ExpiredUncapturedCharge => serializer.serialize_str("expired_uncaptured_charge"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RefundReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "duplicate" => Ok(Self::Duplicate),
            "fraudulent" => Ok(Self::Fraudulent),
            "requested_by_customer" => Ok(Self::RequestedByCustomer),
            "expired_uncaptured_charge" => Ok(Self::ExpiredUncapturedCharge),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RefundReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Duplicate => write!(f, "duplicate"),
            Self::Fraudulent => write!(f, "fraudulent"),
            Self::RequestedByCustomer => write!(f, "requested_by_customer"),
            Self::ExpiredUncapturedCharge => write!(f, "expired_uncaptured_charge"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
