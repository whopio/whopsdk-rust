pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SummaryResolutionCenterCasesRequestReasonItem {
    Fraudulent,
    ProductNotReceived,
    NotAsDescribed,
    ProductUnacceptable,
    SubscriptionCanceled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SummaryResolutionCenterCasesRequestReasonItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Fraudulent => serializer.serialize_str("fraudulent"),
            Self::ProductNotReceived => serializer.serialize_str("product_not_received"),
            Self::NotAsDescribed => serializer.serialize_str("not_as_described"),
            Self::ProductUnacceptable => serializer.serialize_str("product_unacceptable"),
            Self::SubscriptionCanceled => serializer.serialize_str("subscription_canceled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SummaryResolutionCenterCasesRequestReasonItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "fraudulent" => Ok(Self::Fraudulent),
            "product_not_received" => Ok(Self::ProductNotReceived),
            "not_as_described" => Ok(Self::NotAsDescribed),
            "product_unacceptable" => Ok(Self::ProductUnacceptable),
            "subscription_canceled" => Ok(Self::SubscriptionCanceled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SummaryResolutionCenterCasesRequestReasonItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fraudulent => write!(f, "fraudulent"),
            Self::ProductNotReceived => write!(f, "product_not_received"),
            Self::NotAsDescribed => write!(f, "not_as_described"),
            Self::ProductUnacceptable => write!(f, "product_unacceptable"),
            Self::SubscriptionCanceled => write!(f, "subscription_canceled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
