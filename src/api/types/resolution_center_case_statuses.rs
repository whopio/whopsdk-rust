pub use crate::prelude::*;

/// The statuses a resolution object can have
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResolutionCenterCaseStatuses {
    MerchantResponseNeeded,
    CustomerResponseNeeded,
    MerchantInfoNeeded,
    CustomerInfoNeeded,
    UnderPlatformReview,
    CustomerWon,
    MerchantWon,
    CustomerWithdrew,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResolutionCenterCaseStatuses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MerchantResponseNeeded => serializer.serialize_str("merchant_response_needed"),
            Self::CustomerResponseNeeded => serializer.serialize_str("customer_response_needed"),
            Self::MerchantInfoNeeded => serializer.serialize_str("merchant_info_needed"),
            Self::CustomerInfoNeeded => serializer.serialize_str("customer_info_needed"),
            Self::UnderPlatformReview => serializer.serialize_str("under_platform_review"),
            Self::CustomerWon => serializer.serialize_str("customer_won"),
            Self::MerchantWon => serializer.serialize_str("merchant_won"),
            Self::CustomerWithdrew => serializer.serialize_str("customer_withdrew"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResolutionCenterCaseStatuses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "merchant_response_needed" => Ok(Self::MerchantResponseNeeded),
            "customer_response_needed" => Ok(Self::CustomerResponseNeeded),
            "merchant_info_needed" => Ok(Self::MerchantInfoNeeded),
            "customer_info_needed" => Ok(Self::CustomerInfoNeeded),
            "under_platform_review" => Ok(Self::UnderPlatformReview),
            "customer_won" => Ok(Self::CustomerWon),
            "merchant_won" => Ok(Self::MerchantWon),
            "customer_withdrew" => Ok(Self::CustomerWithdrew),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResolutionCenterCaseStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MerchantResponseNeeded => write!(f, "merchant_response_needed"),
            Self::CustomerResponseNeeded => write!(f, "customer_response_needed"),
            Self::MerchantInfoNeeded => write!(f, "merchant_info_needed"),
            Self::CustomerInfoNeeded => write!(f, "customer_info_needed"),
            Self::UnderPlatformReview => write!(f, "under_platform_review"),
            Self::CustomerWon => write!(f, "customer_won"),
            Self::MerchantWon => write!(f, "merchant_won"),
            Self::CustomerWithdrew => write!(f, "customer_withdrew"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
