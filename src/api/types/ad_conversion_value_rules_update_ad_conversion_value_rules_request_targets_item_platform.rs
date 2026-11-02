pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateAdConversionValueRulesRequestTargetsItemPlatform {
    Tiktok,
    Meta,
    Google,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateAdConversionValueRulesRequestTargetsItemPlatform {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Tiktok => serializer.serialize_str("tiktok"),
            Self::Meta => serializer.serialize_str("meta"),
            Self::Google => serializer.serialize_str("google"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateAdConversionValueRulesRequestTargetsItemPlatform {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "tiktok" => Ok(Self::Tiktok),
            "meta" => Ok(Self::Meta),
            "google" => Ok(Self::Google),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateAdConversionValueRulesRequestTargetsItemPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tiktok => write!(f, "tiktok"),
            Self::Meta => write!(f, "meta"),
            Self::Google => write!(f, "google"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
