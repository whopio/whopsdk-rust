pub use crate::prelude::*;

/// The types of responses the platform can make to a resolution.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResolutionCenterCasePlatformResponses {
    RequestBuyerInfo,
    RequestMerchantInfo,
    MerchantWins,
    MerchantRefund,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResolutionCenterCasePlatformResponses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RequestBuyerInfo => serializer.serialize_str("request_buyer_info"),
            Self::RequestMerchantInfo => serializer.serialize_str("request_merchant_info"),
            Self::MerchantWins => serializer.serialize_str("merchant_wins"),
            Self::MerchantRefund => serializer.serialize_str("merchant_refund"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResolutionCenterCasePlatformResponses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "request_buyer_info" => Ok(Self::RequestBuyerInfo),
            "request_merchant_info" => Ok(Self::RequestMerchantInfo),
            "merchant_wins" => Ok(Self::MerchantWins),
            "merchant_refund" => Ok(Self::MerchantRefund),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResolutionCenterCasePlatformResponses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestBuyerInfo => write!(f, "request_buyer_info"),
            Self::RequestMerchantInfo => write!(f, "request_merchant_info"),
            Self::MerchantWins => write!(f, "merchant_wins"),
            Self::MerchantRefund => write!(f, "merchant_refund"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
