pub use crate::prelude::*;

/// The deliverable shapes this bounty accepts. Every bounty accepts any combination of `content_url` (posted links) and `media` (uploaded files), except `data_capture` bounties, whose proof is clips recorded in the Whop app that accumulate on the attempt.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BountyAcceptedDeliverableTypesItem {
    ContentUrl,
    Media,
    DataCapture,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BountyAcceptedDeliverableTypesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ContentUrl => serializer.serialize_str("content_url"),
            Self::Media => serializer.serialize_str("media"),
            Self::DataCapture => serializer.serialize_str("data_capture"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BountyAcceptedDeliverableTypesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "content_url" => Ok(Self::ContentUrl),
            "media" => Ok(Self::Media),
            "data_capture" => Ok(Self::DataCapture),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BountyAcceptedDeliverableTypesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContentUrl => write!(f, "content_url"),
            Self::Media => write!(f, "media"),
            Self::DataCapture => write!(f, "data_capture"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
