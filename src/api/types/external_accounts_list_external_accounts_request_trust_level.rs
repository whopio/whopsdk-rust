pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListExternalAccountsRequestTrustLevel {
    Oauth,
    ApiKey,
    Verified,
    Authorized,
    Claimed,
    Scraped,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListExternalAccountsRequestTrustLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Oauth => serializer.serialize_str("oauth"),
            Self::ApiKey => serializer.serialize_str("api_key"),
            Self::Verified => serializer.serialize_str("verified"),
            Self::Authorized => serializer.serialize_str("authorized"),
            Self::Claimed => serializer.serialize_str("claimed"),
            Self::Scraped => serializer.serialize_str("scraped"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListExternalAccountsRequestTrustLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "oauth" => Ok(Self::Oauth),
            "api_key" => Ok(Self::ApiKey),
            "verified" => Ok(Self::Verified),
            "authorized" => Ok(Self::Authorized),
            "claimed" => Ok(Self::Claimed),
            "scraped" => Ok(Self::Scraped),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListExternalAccountsRequestTrustLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Oauth => write!(f, "oauth"),
            Self::ApiKey => write!(f, "api_key"),
            Self::Verified => write!(f, "verified"),
            Self::Authorized => write!(f, "authorized"),
            Self::Claimed => write!(f, "claimed"),
            Self::Scraped => write!(f, "scraped"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
