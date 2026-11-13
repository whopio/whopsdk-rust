pub use crate::prelude::*;

/// The platform the link points to. `website` is a personal site rather than a social platform.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserSocialLinkWebsite {
    X,
    Instagram,
    Facebook,
    Tiktok,
    Youtube,
    Linkedin,
    Twitch,
    Website,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UserSocialLinkWebsite {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::X => serializer.serialize_str("x"),
            Self::Instagram => serializer.serialize_str("instagram"),
            Self::Facebook => serializer.serialize_str("facebook"),
            Self::Tiktok => serializer.serialize_str("tiktok"),
            Self::Youtube => serializer.serialize_str("youtube"),
            Self::Linkedin => serializer.serialize_str("linkedin"),
            Self::Twitch => serializer.serialize_str("twitch"),
            Self::Website => serializer.serialize_str("website"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UserSocialLinkWebsite {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "x" => Ok(Self::X),
            "instagram" => Ok(Self::Instagram),
            "facebook" => Ok(Self::Facebook),
            "tiktok" => Ok(Self::Tiktok),
            "youtube" => Ok(Self::Youtube),
            "linkedin" => Ok(Self::Linkedin),
            "twitch" => Ok(Self::Twitch),
            "website" => Ok(Self::Website),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UserSocialLinkWebsite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(f, "x"),
            Self::Instagram => write!(f, "instagram"),
            Self::Facebook => write!(f, "facebook"),
            Self::Tiktok => write!(f, "tiktok"),
            Self::Youtube => write!(f, "youtube"),
            Self::Linkedin => write!(f, "linkedin"),
            Self::Twitch => write!(f, "twitch"),
            Self::Website => write!(f, "website"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
