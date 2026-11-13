pub use crate::prelude::*;

/// `active` receives conversions. `errored` stopped because the ad platform rejected the access token; update the token to resume. `disabled` was turned off by Whop.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdPixelStatus {
    Active,
    Errored,
    Disabled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdPixelStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Errored => serializer.serialize_str("errored"),
            Self::Disabled => serializer.serialize_str("disabled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdPixelStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "errored" => Ok(Self::Errored),
            "disabled" => Ok(Self::Disabled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdPixelStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Errored => write!(f, "errored"),
            Self::Disabled => write!(f, "disabled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
