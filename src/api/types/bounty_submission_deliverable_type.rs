pub use crate::prelude::*;

/// How the work arrived when it came in through the API in one shot, informational only — read the work from `deliverable_urls`, `files`, and `capture_clips` directly. `null` for submissions whose proof is a livestream recording, including ones that attached links or files on submit.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BountySubmissionDeliverableType {
    ContentUrl,
    Media,
    DataCapture,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BountySubmissionDeliverableType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ContentUrl => serializer.serialize_str("content_url"),
            Self::Media => serializer.serialize_str("media"),
            Self::DataCapture => serializer.serialize_str("data_capture"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BountySubmissionDeliverableType {
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

impl fmt::Display for BountySubmissionDeliverableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContentUrl => write!(f, "content_url"),
            Self::Media => write!(f, "media"),
            Self::DataCapture => write!(f, "data_capture"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
