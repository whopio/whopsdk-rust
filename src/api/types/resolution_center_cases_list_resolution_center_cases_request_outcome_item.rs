pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListResolutionCenterCasesRequestOutcomeItem {
    CustomerWon,
    MerchantWon,
    Withdrawn,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListResolutionCenterCasesRequestOutcomeItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CustomerWon => serializer.serialize_str("customer_won"),
            Self::MerchantWon => serializer.serialize_str("merchant_won"),
            Self::Withdrawn => serializer.serialize_str("withdrawn"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListResolutionCenterCasesRequestOutcomeItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "customer_won" => Ok(Self::CustomerWon),
            "merchant_won" => Ok(Self::MerchantWon),
            "withdrawn" => Ok(Self::Withdrawn),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListResolutionCenterCasesRequestOutcomeItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CustomerWon => write!(f, "customer_won"),
            Self::MerchantWon => write!(f, "merchant_won"),
            Self::Withdrawn => write!(f, "withdrawn"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
