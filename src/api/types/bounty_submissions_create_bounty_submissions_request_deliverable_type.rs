pub use crate::prelude::*;

/// Legacy shape selector; no longer selects anything. When present it must name an inline shape (`content_url` or `media`).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBountySubmissionsRequestDeliverableType {
    ContentUrl,
    Media,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBountySubmissionsRequestDeliverableType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ContentUrl => serializer.serialize_str("content_url"),
            Self::Media => serializer.serialize_str("media"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBountySubmissionsRequestDeliverableType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "content_url" => Ok(Self::ContentUrl),
            "media" => Ok(Self::Media),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBountySubmissionsRequestDeliverableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContentUrl => write!(f, "content_url"),
            Self::Media => write!(f, "media"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
