pub use crate::prelude::*;

/// Publisher platform where the ad is eligible to appear.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdGroupPlacementPlatform {
    Facebook,
    Instagram,
    Messenger,
    AudienceNetwork,
    Threads,
    Whatsapp,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdGroupPlacementPlatform {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Facebook => serializer.serialize_str("facebook"),
            Self::Instagram => serializer.serialize_str("instagram"),
            Self::Messenger => serializer.serialize_str("messenger"),
            Self::AudienceNetwork => serializer.serialize_str("audience_network"),
            Self::Threads => serializer.serialize_str("threads"),
            Self::Whatsapp => serializer.serialize_str("whatsapp"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdGroupPlacementPlatform {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "facebook" => Ok(Self::Facebook),
            "instagram" => Ok(Self::Instagram),
            "messenger" => Ok(Self::Messenger),
            "audience_network" => Ok(Self::AudienceNetwork),
            "threads" => Ok(Self::Threads),
            "whatsapp" => Ok(Self::Whatsapp),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdGroupPlacementPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Facebook => write!(f, "facebook"),
            Self::Instagram => write!(f, "instagram"),
            Self::Messenger => write!(f, "messenger"),
            Self::AudienceNetwork => write!(f, "audience_network"),
            Self::Threads => write!(f, "threads"),
            Self::Whatsapp => write!(f, "whatsapp"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
