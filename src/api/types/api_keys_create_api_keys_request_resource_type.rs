pub use crate::prelude::*;

/// The type of resource that will own this API key.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateApiKeysRequestResourceType {
    Account,
    App,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateApiKeysRequestResourceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Account => serializer.serialize_str("account"),
            Self::App => serializer.serialize_str("app"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateApiKeysRequestResourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "account" => Ok(Self::Account),
            "app" => Ok(Self::App),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateApiKeysRequestResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Account => write!(f, "account"),
            Self::App => write!(f, "app"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
