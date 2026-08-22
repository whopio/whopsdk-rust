pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SummaryResolutionCenterCasesRequestStatusItem {
    AwaitingMerchant,
    AwaitingCustomer,
    UnderReview,
    Closed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SummaryResolutionCenterCasesRequestStatusItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AwaitingMerchant => serializer.serialize_str("awaiting_merchant"),
            Self::AwaitingCustomer => serializer.serialize_str("awaiting_customer"),
            Self::UnderReview => serializer.serialize_str("under_review"),
            Self::Closed => serializer.serialize_str("closed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SummaryResolutionCenterCasesRequestStatusItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "awaiting_merchant" => Ok(Self::AwaitingMerchant),
            "awaiting_customer" => Ok(Self::AwaitingCustomer),
            "under_review" => Ok(Self::UnderReview),
            "closed" => Ok(Self::Closed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SummaryResolutionCenterCasesRequestStatusItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AwaitingMerchant => write!(f, "awaiting_merchant"),
            Self::AwaitingCustomer => write!(f, "awaiting_customer"),
            Self::UnderReview => write!(f, "under_review"),
            Self::Closed => write!(f, "closed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
