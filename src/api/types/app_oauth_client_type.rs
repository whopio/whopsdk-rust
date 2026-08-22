pub use crate::prelude::*;

/// How the app authenticates at the OAuth token endpoint.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppOauthClientType {
    Public,
    Confidential,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AppOauthClientType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Public => serializer.serialize_str("public"),
            Self::Confidential => serializer.serialize_str("confidential"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AppOauthClientType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "public" => Ok(Self::Public),
            "confidential" => Ok(Self::Confidential),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AppOauthClientType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::Confidential => write!(f, "confidential"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
