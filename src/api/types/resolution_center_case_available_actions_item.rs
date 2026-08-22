pub use crate::prelude::*;

/// What you can do to this case right now, named for the endpoint that does it. Resolved for the calling credential, so a merchant and a customer reading the same case see their own options.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResolutionCenterCaseAvailableActionsItem {
    Accept,
    Deny,
    RequestInfo,
    Reply,
    Appeal,
    Withdraw,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResolutionCenterCaseAvailableActionsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Accept => serializer.serialize_str("accept"),
            Self::Deny => serializer.serialize_str("deny"),
            Self::RequestInfo => serializer.serialize_str("request_info"),
            Self::Reply => serializer.serialize_str("reply"),
            Self::Appeal => serializer.serialize_str("appeal"),
            Self::Withdraw => serializer.serialize_str("withdraw"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResolutionCenterCaseAvailableActionsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "accept" => Ok(Self::Accept),
            "deny" => Ok(Self::Deny),
            "request_info" => Ok(Self::RequestInfo),
            "reply" => Ok(Self::Reply),
            "appeal" => Ok(Self::Appeal),
            "withdraw" => Ok(Self::Withdraw),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResolutionCenterCaseAvailableActionsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accept => write!(f, "accept"),
            Self::Deny => write!(f, "deny"),
            Self::RequestInfo => write!(f, "request_info"),
            Self::Reply => write!(f, "reply"),
            Self::Appeal => write!(f, "appeal"),
            Self::Withdraw => write!(f, "withdraw"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
