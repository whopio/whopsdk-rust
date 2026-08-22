pub use crate::prelude::*;

/// The types of responses a customer can make to a resolution.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResolutionCenterCaseCustomerResponses {
    Respond,
    Appeal,
    Withdraw,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResolutionCenterCaseCustomerResponses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Respond => serializer.serialize_str("respond"),
            Self::Appeal => serializer.serialize_str("appeal"),
            Self::Withdraw => serializer.serialize_str("withdraw"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResolutionCenterCaseCustomerResponses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "respond" => Ok(Self::Respond),
            "appeal" => Ok(Self::Appeal),
            "withdraw" => Ok(Self::Withdraw),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResolutionCenterCaseCustomerResponses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Respond => write!(f, "respond"),
            Self::Appeal => write!(f, "appeal"),
            Self::Withdraw => write!(f, "withdraw"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
