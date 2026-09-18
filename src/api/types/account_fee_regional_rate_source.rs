pub use crate::prelude::*;

/// Where the regional rate in effect comes from: `default` is the platform rate, `custom` a rate negotiated for this account, and `inherited` a rate negotiated by the platform this account is connected to.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountFeeRegionalRateSource {
    Default,
    Custom,
    Inherited,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountFeeRegionalRateSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Default => serializer.serialize_str("default"),
            Self::Custom => serializer.serialize_str("custom"),
            Self::Inherited => serializer.serialize_str("inherited"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountFeeRegionalRateSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "default" => Ok(Self::Default),
            "custom" => Ok(Self::Custom),
            "inherited" => Ok(Self::Inherited),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountFeeRegionalRateSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Custom => write!(f, "custom"),
            Self::Inherited => write!(f, "inherited"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
