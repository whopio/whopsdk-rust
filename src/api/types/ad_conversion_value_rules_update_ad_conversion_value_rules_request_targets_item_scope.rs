pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateAdConversionValueRulesRequestTargetsItemScope {
    Business,
    Campaign,
    AdGroup,
    Ad,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateAdConversionValueRulesRequestTargetsItemScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Business => serializer.serialize_str("business"),
            Self::Campaign => serializer.serialize_str("campaign"),
            Self::AdGroup => serializer.serialize_str("ad_group"),
            Self::Ad => serializer.serialize_str("ad"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateAdConversionValueRulesRequestTargetsItemScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "business" => Ok(Self::Business),
            "campaign" => Ok(Self::Campaign),
            "ad_group" => Ok(Self::AdGroup),
            "ad" => Ok(Self::Ad),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateAdConversionValueRulesRequestTargetsItemScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Business => write!(f, "business"),
            Self::Campaign => write!(f, "campaign"),
            Self::AdGroup => write!(f, "ad_group"),
            Self::Ad => write!(f, "ad"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
