pub use crate::prelude::*;

/// The funding source kind.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdatePreferencesRequestAdsPaymentMethodsPrimaryType {
    PlatformBalance,
    Card,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdatePreferencesRequestAdsPaymentMethodsPrimaryType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PlatformBalance => serializer.serialize_str("platform_balance"),
            Self::Card => serializer.serialize_str("card"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdatePreferencesRequestAdsPaymentMethodsPrimaryType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "platform_balance" => Ok(Self::PlatformBalance),
            "card" => Ok(Self::Card),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdatePreferencesRequestAdsPaymentMethodsPrimaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlatformBalance => write!(f, "platform_balance"),
            Self::Card => write!(f, "card"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
