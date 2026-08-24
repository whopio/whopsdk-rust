pub use crate::prelude::*;

/// The types of responses a merchant can make to a resolution.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResolutionCenterCaseMerchantResponses {
    Accept,
    Deny,
    RequestMoreInfo,
    Appeal,
    Respond,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResolutionCenterCaseMerchantResponses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Accept => serializer.serialize_str("accept"),
            Self::Deny => serializer.serialize_str("deny"),
            Self::RequestMoreInfo => serializer.serialize_str("request_more_info"),
            Self::Appeal => serializer.serialize_str("appeal"),
            Self::Respond => serializer.serialize_str("respond"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResolutionCenterCaseMerchantResponses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "accept" => Ok(Self::Accept),
            "deny" => Ok(Self::Deny),
            "request_more_info" => Ok(Self::RequestMoreInfo),
            "appeal" => Ok(Self::Appeal),
            "respond" => Ok(Self::Respond),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResolutionCenterCaseMerchantResponses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accept => write!(f, "accept"),
            Self::Deny => write!(f, "deny"),
            Self::RequestMoreInfo => write!(f, "request_more_info"),
            Self::Appeal => write!(f, "appeal"),
            Self::Respond => write!(f, "respond"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
