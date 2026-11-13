pub use crate::prelude::*;

/// The platform the parent external account exists on.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExternalAccountParentPlatform {
    X,
    Instagram,
    Youtube,
    Tiktok,
    Facebook,
    Linkedin,
    Snapchat,
    Klaviyo,
    Quickbooks,
    Aimerce,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExternalAccountParentPlatform {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::X => serializer.serialize_str("x"),
            Self::Instagram => serializer.serialize_str("instagram"),
            Self::Youtube => serializer.serialize_str("youtube"),
            Self::Tiktok => serializer.serialize_str("tiktok"),
            Self::Facebook => serializer.serialize_str("facebook"),
            Self::Linkedin => serializer.serialize_str("linkedin"),
            Self::Snapchat => serializer.serialize_str("snapchat"),
            Self::Klaviyo => serializer.serialize_str("klaviyo"),
            Self::Quickbooks => serializer.serialize_str("quickbooks"),
            Self::Aimerce => serializer.serialize_str("aimerce"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExternalAccountParentPlatform {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "x" => Ok(Self::X),
            "instagram" => Ok(Self::Instagram),
            "youtube" => Ok(Self::Youtube),
            "tiktok" => Ok(Self::Tiktok),
            "facebook" => Ok(Self::Facebook),
            "linkedin" => Ok(Self::Linkedin),
            "snapchat" => Ok(Self::Snapchat),
            "klaviyo" => Ok(Self::Klaviyo),
            "quickbooks" => Ok(Self::Quickbooks),
            "aimerce" => Ok(Self::Aimerce),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExternalAccountParentPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(f, "x"),
            Self::Instagram => write!(f, "instagram"),
            Self::Youtube => write!(f, "youtube"),
            Self::Tiktok => write!(f, "tiktok"),
            Self::Facebook => write!(f, "facebook"),
            Self::Linkedin => write!(f, "linkedin"),
            Self::Snapchat => write!(f, "snapchat"),
            Self::Klaviyo => write!(f, "klaviyo"),
            Self::Quickbooks => write!(f, "quickbooks"),
            Self::Aimerce => write!(f, "aimerce"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
