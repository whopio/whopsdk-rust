pub use crate::prelude::*;

/// The platform to connect the external account on. Use `tiktok` to connect the authenticated user’s TikTok profile, `linkedin` for their LinkedIn profile, `youtube` for their YouTube channel, or `x` for their X profile. Use `quickbooks` to connect the account’s QuickBooks Online company: pass `account_id`, and the user picks the company in QuickBooks.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectExternalAccountsRequestPlatform {
    Tiktok,
    Linkedin,
    Youtube,
    X,
    Quickbooks,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConnectExternalAccountsRequestPlatform {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Tiktok => serializer.serialize_str("tiktok"),
            Self::Linkedin => serializer.serialize_str("linkedin"),
            Self::Youtube => serializer.serialize_str("youtube"),
            Self::X => serializer.serialize_str("x"),
            Self::Quickbooks => serializer.serialize_str("quickbooks"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConnectExternalAccountsRequestPlatform {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "tiktok" => Ok(Self::Tiktok),
            "linkedin" => Ok(Self::Linkedin),
            "youtube" => Ok(Self::Youtube),
            "x" => Ok(Self::X),
            "quickbooks" => Ok(Self::Quickbooks),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConnectExternalAccountsRequestPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tiktok => write!(f, "tiktok"),
            Self::Linkedin => write!(f, "linkedin"),
            Self::Youtube => write!(f, "youtube"),
            Self::X => write!(f, "x"),
            Self::Quickbooks => write!(f, "quickbooks"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
