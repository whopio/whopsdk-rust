pub use crate::prelude::*;

/// The platform to create the social account on. `facebook` requires the account's `banner_image`, `logo`, and `description`, and `tiktok` requires its `logo`; configure them with [Update Account](/api-reference/beta/accounts/update-account). The account is returned before the platform has created it — its `id` is usable right away, and the rest of the profile fills in once provisioning finishes.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateSocialAccountsRequestPlatform {
    Facebook,
    Tiktok,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateSocialAccountsRequestPlatform {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Facebook => serializer.serialize_str("facebook"),
            Self::Tiktok => serializer.serialize_str("tiktok"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateSocialAccountsRequestPlatform {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "facebook" => Ok(Self::Facebook),
            "tiktok" => Ok(Self::Tiktok),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateSocialAccountsRequestPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Facebook => write!(f, "facebook"),
            Self::Tiktok => write!(f, "tiktok"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
