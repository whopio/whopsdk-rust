pub use crate::prelude::*;

/// The API version used to format payloads sent to this webhook endpoint.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookListItemApiVersion {
    V1,
    V2,
    V5,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhookListItemApiVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::V1 => serializer.serialize_str("v1"),
            Self::V2 => serializer.serialize_str("v2"),
            Self::V5 => serializer.serialize_str("v5"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookListItemApiVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "v1" => Ok(Self::V1),
            "v2" => Ok(Self::V2),
            "v5" => Ok(Self::V5),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhookListItemApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V1 => write!(f, "v1"),
            Self::V2 => write!(f, "v2"),
            Self::V5 => write!(f, "v5"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
